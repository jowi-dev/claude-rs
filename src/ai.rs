use reqwest::header;
use reqwest;
use serde_json::json;
use serde_json;
use std::{env, vec};

use rusqlite::{Connection, Result};
use crate::storage::Storable;

#[derive(Debug)]
pub struct AILog {
    pub role: String, 
    pub content: String
}

impl Storable<AILog> for AILog {

    fn new(role: &String, content: &String) -> AILog {
        AILog{ role: role.to_string(), content: content.to_string() }
    }
    fn create(&self, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute( " INSERT INTO ai_logs (role, content) VALUES (?1, ?2) ", (self.role.to_string(), self.content.to_string()))?;

        Ok(())
    }

    fn init(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute("
            CREATE TABLE IF NOT EXISTS ai_logs (
                id INTEGER primary key,
                role text ,
                content text 
            )", ())?; 

        Ok(())
    }

    fn all(conn: &Connection) -> Result<Vec<AILog>, Box<dyn std::error::Error>> {
        let mut statement = conn.prepare("SELECT role, content from ai_logs;")?;
        let log_iter = statement.query_map([], |row| {
            Ok(AILog {
                role: row.get(0)?,
                content: row.get(1)?
            })
        })?;

        let mut logs :Vec<AILog> = Vec::new();
        for log in log_iter.collect::<Vec<Result<AILog, rusqlite::Error>>>() {
            logs.push(log.unwrap());
        }



        Ok(logs)
    }

}

pub fn get_logs() -> Result<Vec<AILog>, Box<dyn std::error::Error>> {
    let conn = Connection::open("./test.db".to_string())?;
    let results : Vec<AILog> = AILog::all(&conn)?;

    for result in results.iter() {
        let lines :Vec<&str> = result.content
            .split("\\n")
            .collect();

        for line in lines.iter() {
            println!("{}", line);
        }

    }
    Ok(results)
}

pub async fn request(prompt: String, is_test: bool) -> Result<(String,String), Box<dyn std::error::Error>> {
    if is_test && prompt == "Fail".to_string() {
        return Err("failed to request".into());
    } 

    // Testing only, but sanity for not hammering the API and wasting money
    if is_test && prompt != "Fail".to_string() {
        return Ok(("assistant".to_string(), "Hello World".to_string()));
    }         

    return Ok(anthropic_request(prompt).await?);
}

async fn anthropic_request(prompt: String) -> Result<(String,String), Box<dyn std::error::Error>>{
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found in env");
    let headers = build_headers(api_key);

    let body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });

    let conn = Connection::open("./test.db".to_string())?;

    let _ = AILog::init(&conn).expect("Failed to create ai_logs");

    let user_log = AILog::new(&"user".to_string(), &prompt.to_string());
    let _ = AILog::create(&user_log, &conn).expect("Failed to insert record");


    // Make the request Client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Do the cha cha
    let resp = client 
        .post("https://api.anthropic.com/v1/messages")
        .json(&body)
        .send()
        .await?
        .text()
        .await? ;

    let v :serde_json::Value = serde_json::from_str(&resp).unwrap();

    let role : String = "assistant".to_string();
    let message :String = v["content"][0]["text"].to_string();

    let log = AILog::new(&role, &message);
    let _ = AILog::create(&log, &conn);

    Ok((role,message))
}

fn build_headers(api_key: String) -> header::HeaderMap {
    let api_key_header = header::HeaderValue::from_str(&api_key).unwrap();

    // Header Setup
    let mut headers = header::HeaderMap::new();
    headers.insert("x-api-key", api_key_header);
    headers.insert(
        "anthropic-version",
        header::HeaderValue::from_static("2023-06-01"),
    );
    headers.insert(
        "content-type",
        header::HeaderValue::from_static("application/json"),
    );

    headers
}
