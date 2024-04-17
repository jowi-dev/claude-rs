use reqwest::header;
use reqwest;
use serde_json::json;
use serde_json;
use std::env;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct AILog {
    role: String, 
    content: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let conn = Connection::open("./test.db".to_string())?;
    let prompt = "Hello! do you think its valuable to save responses you send me in a sqlite database? What could I do with them?";
    let (role, message) = request(prompt.to_string(), true).await?;
    let _ = insert_record(&conn, &role, &message);
    return read_from_logs(&conn)
}

async fn request(prompt: String, is_test: bool) -> Result<(String,String), Box<dyn std::error::Error>> {
    if is_test && prompt == "Fail".to_string() {
        return Err("failed to request".into());
    } 

    if is_test && prompt != "Fail".to_string() {
        return Ok(("assistant".to_string(), "Hello World".to_string()));
    }         

    return Ok(anthropic_request(prompt).await?);
}

//#[tokio::main]
async fn anthropic_request(prompt: String) -> Result<(String,String), Box<dyn std::error::Error>>{
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not found in env");
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

    let body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });


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

    Ok((role,message))
}

fn insert_record(conn: &Connection, role: &String, content: &String) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS ai_logs (
            id INTEGER primary key,
            role text ,
            content text 
        )
        ", ()
    )?; 

    conn.execute(
        "
        INSERT INTO ai_logs (role, content) VALUES (?1, ?2)
        ", 
        (role, content)
    )?;

    Ok(())

}

fn read_from_logs(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut statement = conn.prepare("SELECT role, content from ai_logs;")?;
    let log_iter = statement.query_map([], |row| {
        Ok(AILog {
            role: row.get(0)?,
            content: row.get(1)?
        })
    })?;

    for log in log_iter {
        println!("Log: {:?}", log.unwrap());
    };

    Ok(())
}
