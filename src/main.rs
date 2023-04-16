use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, error::Error};
use tokio;

#[derive(Deserialize, Serialize, Debug)]
struct StreamMsg {
    id: String,
    model: String,
    choices: Vec<StreamMsgChoice>,
}

#[derive(Deserialize, Serialize, Debug)]
struct StreamMsgChoiceDelta {
    content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct StreamMsgChoice {
    delta: StreamMsgChoiceDelta,
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Msg {
    role: String,
    content: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Obj {
    model: String,
    messages: Vec<Msg>,
    stream: bool,
}

async fn post(context: &str, message: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let data = r#"
   {
    "model": "gpt-4",
    "stream": true,
    "messages": [{"role": "system", "content": "always answer in rhyme"},{"role": "user", "content": "Hello!"}]
   }"#;

    let message = Msg {
        role: "system".to_string(),
        content: message,
    };
    let data_package = Obj{ model:"gpt-4".to_string(), stream: true, messages:vec!(
            Msg{ role: "system".to_string(), content: match context {
                "dnd" => "You are an assistant to a DM and you help him make a great epic fantasy setting d&d campaign. Use full dnd statblocks and CR for characters. Full item descriptions like d&d rules for items. Always use elequent prose and try to add multiple senses to descriptions of scenarios and scenes." ,
                "code" => "You are an assistant coder. Prefered languges are in order: Rust, typescript, c#, python. Prefered frameworks: solid.js, tailwind, svelte, react, preact, angular. answer with code blocks and limited prose. I don't like to read text but I can scan code quickly",
                _ => "There is no prior context. Be your awesome you!",
                }.to_string()
            }, message)};

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

                        print!("{}", message)
                    }
                    Err(e) => println!("{:#?}, {:#?}", e, trimmed),
                }
            }
        });

    }
    //println!("collected: {}", collected_message);
    let abriviation_message = Obj{
       stream: false,
        model: "gpt-3.5".to_string(),
        messages: vec!(Msg{role: "system".to_string(), content:"Abreviate this following message to the best possble short (limited token) use, so that I can use it as a history for the context in gpt prompting".to_string()})
    }; 
    let abr_request = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&key)
        .json(&data_package);
    let abr_res = abr_request.send().await?;
    println!("abr response: {:#?}", abr_res);

    Ok(format!("res: {:#?}", data))
}
use clap::Parser;

/// Simple program to query gpt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the context to use
    #[arg(short, long)]
    context: String,

    /// Number of times to greet
    #[arg(short, long )]
    message: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _res = post(args.context.as_str(), args.message).await;
}
