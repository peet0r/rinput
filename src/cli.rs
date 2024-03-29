use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug, Parser)]
#[command(name = "rinput")]
#[command(about  = "record and replay libinput events, you might need to run with \"sudo\"", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: RInputCommand,
}

#[derive(Debug, Subcommand)]
pub enum RInputCommand {
    Record(Record),
    Replay(Replay),
    Generate(Generate),
}

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "Record input events from a device, you might need to run with \"sudo\""
)]
pub struct Record {
    #[arg(short, long, help = "Filepath to save JSON output to")]
    pub output: Option<String>,
    #[arg(short, long, help = "List all connected devices, then exit")]
    pub enumerate: bool,
}

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "Replay input events from a source file, you might need to run with \"sudo\""
)]
pub struct Replay {
    #[arg(short, long, help = "Filepath to source for event timeline")]
    pub source: String,
    #[arg(short, long, help = "Scaling factor for time", default_value_t = 1.0)]
    pub factor: f32,
    #[arg(
        short = 'q',
        long,
        help = "Send sequence on term input. if false, run once"
    )]
    pub sequence: bool,
}

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "Replay input events from a source file, you might need to run with \"sudo\""
)]
pub struct Generate {
    #[arg(short, long, help = "Filepath to save JSON output to")]
    pub output: Option<String>,
    #[arg(short, long, help = "Filepath to source for event timeline")]
    pub source: String,
    #[arg(
        short,
        long,
        help = "Time delta for each event [milliseconds]",
        default_value_t = 1000
    )]
    pub delta: u64,
    #[arg(
        short = 'q',
        long,
        help = "Send sequence on term input. if false, run once"
    )]
    pub sequence: bool,
    #[arg(
        short,
        long,
        help = "Add initial delay prior to starting the sequence. minimum value is 1000. [milliseconds]",
        default_value_t = 1000
    )]
    pub initialdelay: u128,
}

// TODO: This should be a result
pub fn pick_device(list: Vec<(String, String)>) -> Result<String> {
    let names: Vec<String> = list.iter().map(|t| t.1.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a device to record from:")
        .items(&names)
        .default(0)
        .interact_on_opt(&Term::stdout())
        .unwrap();

    if let Some(i) = selection {
        Ok(list[i].0.clone())
    } else {
        Err(anyhow!("Not a valid selection"))
    }
}
