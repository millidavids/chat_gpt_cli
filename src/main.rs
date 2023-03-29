mod args;
mod chat;

use reqwest::header::CONTENT_TYPE;

use crate::args::Args;
use crate::chat::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = <Args as clap::Parser>::parse();

    let _convo = if let Some(convo_name) = args.convo {
        create_chat_dir()?;
        Some(Convo::fetch_or_create(&convo_name)?)
    } else {
        None
    };
    // let crb = ChatRequestBody {
    //     messages: vec![Message {
    //         content: args.prompt.unwrap(),
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };
    // let crb_json = serde_json::to_string(&crb).unwrap();

    // let client = reqwest::blocking::Client::new();
    // let res = client
    //     .post("https://api.openai.com/v1/chat/completions")
    //     .header(CONTENT_TYPE, "application/json")
    //     .bearer_auth(std::env::var("OPENAI_API_KEY").unwrap())
    //     .body(crb_json)
    //     .send()
    //     .unwrap();

    // let cr: ChatResponse = serde_json::from_str(&res.text().unwrap()[..]).unwrap();

    // println!("{:?}", cr);

    Ok(())
}

fn create_chat_dir() -> std::io::Result<()> {
    if let Some(mut dir_buf) = dirs::home_dir() {
        dir_buf.push(".convos");
        std::fs::create_dir_all(dir_buf.as_path())?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Unable to find home dir.",
        ))
    }
}
