use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about = "A CLI for interacting with ChatGPT")]
pub struct Args {
    #[arg(short, long)]
    /// The prompt to which ChatGPT will respond 
    pub prompt: Option<String>,
    #[arg(short, long)]
    /// A conversation that you can save and continue
    pub convo: Option<String>,
    #[arg(short = 'H', long, default_value = "false")]
    /// Print chat history
    pub history: Option<bool>,
}