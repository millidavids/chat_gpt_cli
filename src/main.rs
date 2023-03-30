mod args;
mod chat;
mod util;

use crate::args::Args;
use crate::chat::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = <Args as clap::Parser>::parse();

    let convo = if let Some(convo_name) = args.convo {
        create_chat_dir()?;
        Some(Convo::fetch_or_create(&convo_name)?)
    } else {
        None
    };

    match (convo, args.prompt) {
        (Some(mut convo), Some(prompt)) => {
            convo.messages.push(Message {
                content: prompt,
                ..Default::default()
            });
            let crb = ChatRequestBody {
                messages: convo.messages.clone(),
                ..Default::default()
            };
            let crb_json = serde_json::to_string(&crb)?;
            let cr = ChatResponse::from_api(crb_json)?;
            convo.messages.push(cr.choices[0].message.clone());
            convo.save()?;
            if args.history.unwrap() == true {
                for message in convo.messages {
                    println!(
                        "---------------\nRole: {:?}\n---------------\n{}\n",
                        message.role, message.content
                    );
                }
            } else {
                let message = &convo.messages[convo.messages.len() - 1];
                println!(
                    "---------------\nRole: {:?}\n---------------\n{}\n",
                    message.role, message.content,
                );
            }
        }
        (Some(convo), None) => {
            if args.history.unwrap() == true {
                for message in convo.messages {
                    println!(
                        "---------------\nRole: {:?}\n---------------\n{}\n",
                        message.role, message.content
                    );
                }
            }
        }
        (None, Some(prompt)) => {
            let crb = ChatRequestBody {
                messages: vec![Message {
                    content: prompt,
                    ..Default::default()
                }],
                ..Default::default()
            };
            let crb_json = serde_json::to_string(&crb)?;
            let cr = ChatResponse::from_api(crb_json)?;
            println!(
                "---------------\nRole: {:?}\n---------------\n{}\n",
                cr.choices[0].message.role, cr.choices[0].message.content
            );
        }
        (None, None) => (),
    }

    Ok(())
}

fn create_chat_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(util::assemble_convo_path()?)?;
    Ok(())
}
