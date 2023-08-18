mod contexts;
mod file;
mod model;

use clap::Parser;
use futures::StreamExt;
use reqwest::Client;
use serde_json;
use std::io;
use std::{env, error::Error, io::Write};
use tokio;

use crate::{
    contexts::{ABREVIATE, CODE, DND, SHORT},
    file::{append_data, append_log, load_data},
    model::{ChatCompletion, Msg, Obj, StreamMsg},
};

async fn prompt(context: &str, message: &String, history: &Vec<String>) -> Result<String, Box<dyn Error>> {
    let client = Client::new();


    let user_message = Msg {
        role: "user".to_string(),
        content: message.clone(),
    };

    let mut messages: Vec<Msg> = history
        .iter()
        .map(|h| Msg {
            role: "assistant".to_string(),
            content: h.clone(),
        })
        .collect();

    messages.push(Msg {
        role: "system".to_string(),
        content: match context {
            "dnd" => DND,
            "code" => CODE,
            "short" => SHORT,
            _ => "",
        }
        .to_string(),
    });
    messages.push(Msg {
        role: "system".to_string(),
        content: "At the end of the answer add a weigth in form of W:<1-10> for how important this information is to the greater context".to_string()
    });

    messages.push(user_message);

    let data_package = Obj {
        model: "gpt-4".to_string(),
        stream: true,
        messages,
    };

    let key = env::var("OPENAI_API_KEY")?;

    let request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&key)
        .json(&data_package);

    let res = request.send().await?;
    let mut stream = res.bytes_stream();

    let mut collected_message = "".to_string();
    println!();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        let bytes = chunk.to_vec();

        let str = String::from_utf8(bytes).clone()?;
        str.split("\n\n").for_each(|s| {
            let trimmed = s.trim_end();
            if trimmed.len() > 6 && trimmed != "data: [DONE]" {
                let msg = serde_json::from_str::<StreamMsg>(&trimmed[6..]);
                match msg {
                    Ok(stream_msg) => {
                        let message = stream_msg
                            .choices
                            .first()
                            .expect("should have content")
                            .delta
                            .content
                            .clone()
                            .or(Some("".to_string()))
                            .unwrap();

                        collected_message.push_str(message.as_str());

                        termimad::print_inline(message.as_str());
                        let _result = io::stdout().flush();
                    }
                    Err(e) => println!("{:#?}, {:#?}", e, trimmed),
                }
            }
        });
    }

    return Ok(collected_message);
}

async fn create_context(
    question: &String,
    message: &String,
) -> Result<String, Box<dyn Error>> {
    let key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let abriviation_message = Obj {
        stream: false,
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Msg {
                role: "system".to_string(),
                content: ABREVIATE.to_string(),
            },
            Msg {
                role: "user".to_string(),
                content: format!("Q:{},A:{}", question, message),
            },
        ],
    };

    let abr_request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&key)
        .json(&abriviation_message);
    let abr_res = abr_request.send().await?;

    let completion: ChatCompletion = abr_res.json().await?;

    return Ok(completion.choices.first().unwrap().message.content.clone());
}

/// Simple program to query gpt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the context to use
    #[arg(short, long)]
    context: String,

    /// Number of times to greet
    #[arg(short, long)]
    message: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let history = load_data(&args.context);
    //println!("using history {:?}", history);
    termimad::print_inline("prompting **gpt-4** and abreviating with **gpt-3.5-turbo**");
    let answer = prompt(args.context.as_str(), &args.message, &history).await;
    match answer {
        Ok(answer_text) => {
            append_log(&args.message, &answer_text, &args.context.to_string());
            let abreviated_answer =
                create_context(&args.message, &answer_text).await;
            match abreviated_answer {
                Ok(abreviated_text) => {
                    append_data(abreviated_text, args.context.to_string());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
