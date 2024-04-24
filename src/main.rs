mod storage;
mod ai;

use ai::AILog;

use clap::{arg, Command, ArgAction};


fn cli() -> Command {
    Command::new("banj")
        .subcommand(
            Command::new("ai")
            .about("ai <PROMPT> will send prompt to claude and print the response")
            .arg(arg!(<PROMPT> "The prompt to send"))
            .args_conflicts_with_subcommands(true)
            .subcommand(Command::new("logs").long_flag("logs"))
            .subcommand(Command::new("test").long_flag("test"))
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("ai", submatches)) => {
            if submatches.get_one::<String>("flag").is_some() {
                let logs : Vec<AILog> = ai::get_logs().expect("database connection could not be established");
                for log in logs {
                    println!("{}", log.content);
                }

            }
            else {
                let is_test = submatches.contains_id("test");

                let prompt = submatches.get_one::<String>("PROMPT").expect("required");

                let (_, content) = ai::request(prompt.to_string(), is_true(is_test.to_string())).await?;
                println!("{}", content);
            }


        }
        _ => unreachable!()
    }
    //let (role, message) = ai::request(prompt.to_string(), true).await?;
    Ok(())
}

fn is_true(flag: String) -> bool {
    flag == "true".to_string() 
}


