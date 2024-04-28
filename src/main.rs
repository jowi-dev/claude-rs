mod storage;
mod ai;

use std::env;
use std::collections::VecDeque;
use ai::AILog;

enum ValidCommand {
    AI,
    //Rebuild
}


struct Command {
    name: ValidCommand,
    input: String, 
    flags: Vec<String>
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let command: Command = parse_command().unwrap();

    match command.name {
        ValidCommand::AI => {
            let mut is_test = false;
            let mut get_logs = false;
            for flag in command.flags.iter() {
                match flag.as_str() {
                    "--test" => is_test = true,
                    "--get-logs" => get_logs = true,
                    flag => panic!("Wtf is this {}", flag.to_string())
                }
            }

            if get_logs {
               let _ =  ai::get_logs().expect("database connection could not be established");
            }
            else {
                let (_, content) = ai::request(command.input.to_string(), is_true(is_test.to_string())).await?;
                println!("{}", content);
            }
        }

    }
    Ok(())
}

fn parse_command() -> Result<Command, Box<dyn std::error::Error>>{
    let mut args : VecDeque<String> = env::args().collect();
    // invocation command
    args.pop_front();
    let mut flags : Vec<String> = vec!();
    let mut input : String = "".to_string();
    dbg!(&args);
    let name = match args.pop_front().expect("Command not specified").as_str() {
        "ai" => ValidCommand::AI,
            _ => todo!()
    };

    for arg in args.iter() {
        if arg.as_str().starts_with("--") {
            flags.push(arg.to_string());
        } else {
            input = arg.to_string();
        }
    }


    let command  = Command{
        name: name, 
        input: input,
        flags: flags,
    };
    Ok(command)
}

fn is_true(flag: String) -> bool {
    flag == "true".to_string() 
}


