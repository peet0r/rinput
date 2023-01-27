use anyhow::Result;
use clap::Parser;
use console::{Style, Term};
use descriptor::{create_device_descriptor, EventDescriptor};
use evdev::uinput::VirtualDevice;
use generate::parse_file;
use std::process::exit;
use tokio::sync::mpsc;
mod cli;
mod descriptor;
mod generate;
mod record;
mod replay;
mod utils;

use crate::descriptor::{Recording, Timeline};
use crate::replay::{replay_in_loop, replay_timeline};
use cli::{Cli, Generate, RInputCommand, Record, Replay};
use record::{enumerate_devices, listen_loop, record_loop, select_device, validate_path, Msg};

const TIMING_OFFSET: f64 = 1.1;
const MILLIS_TO_MICROS: f64 = 1.0;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Term utils could be moved to a util
    let term = Term::stdout();
    let yellow = Style::new().yellow();
    term.write_line(&format!(
        "{} This tool only works for KEY events",
        yellow.apply_to("WARNING:")
    ))?;

    let res = match args.command {
        RInputCommand::Record(rec) => process_record(rec).await,
        RInputCommand::Replay(rep) => process_replay(rep).await,
        RInputCommand::Generate(gen) => process_generate(gen).await,
    };

    if res.is_ok() {
        let green = Style::new().green();
        term.write_line(&format!("Finished: {}", green.apply_to("OK")))?;
    } else {
        let red = Style::new().red();
        term.write_line(&format!(
            "{} : {}",
            red.apply_to("ERROR:"),
            res.err().unwrap()
        ))?;
        term.write_line("Exiting")?;
    }

    Ok(())
}

async fn process_record(rec: Record) -> Result<()> {
    // If enumerate, display devices and exit
    if rec.enumerate {
        enumerate_devices()?;
        return Ok(());
    }

    // Check that output file path is valid
    let output_path = validate_path(rec.output.unwrap())?;

    let device_path = select_device()?;
    let (tx, rx) = mpsc::channel(100);

    let exit = tx.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        exit.send(Msg::Exit)
            .await
            .expect("could not send exit command");
    });

    let _ = tokio::spawn(listen_loop(tx, device_path));

    let record_handle = tokio::spawn(record_loop(rx, output_path));

    let _ = record_handle.await?;

    Ok(())
}

async fn process_replay(rep: Replay) -> Result<()> {
    println!("{:?}", rep);

    // Deserialize JSON file
    let rec: Recording = serde_json::from_str(std::fs::read_to_string(rep.source)?.as_str())?;
    let device = VirtualDevice::try_from(rec.device);
    if device.is_err() {
        println!("You probably need to run this with sudo permissions");
        exit(1);
    }
    let mut device = device.unwrap();

    let timeline: Timeline = rec.event_list.into();

    if rep.sequence {
        replay_in_loop(&mut device, &timeline)?;
    } else {
        replay_timeline(&mut device, &timeline)?;
    }

    Ok(())
}

async fn process_generate(gen: Generate) -> Result<()> {
    // Iterate line by line
    let keys = parse_file(gen.source)?;

    // Generate device that can submit keys
    let desc = create_device_descriptor(keys.clone())?;

    // Create Recording
    let mut rec = Recording {
        device: desc,
        event_list: Vec::new(),
    };

    let mut delta = if gen.initialdelay < 1000 {1000} else { gen.initialdelay};

    for t in keys.iter().enumerate() {
        delta += (gen.delta * (t.0 as u64)) as u128;
        let key = t.1;
        //Down
        rec.event_list
            .push(EventDescriptor::new(delta.into(), 1, key.0, 1));
        //Up
        rec.event_list.push(EventDescriptor::new(
            (delta as f64 * TIMING_OFFSET * MILLIS_TO_MICROS).ceil() as u128,
            1,
            key.0,
            0,
        ));
    }

    // Play recording
    let device = VirtualDevice::try_from(rec.device);
    if device.is_err() {
        println!("You probably need to run this with sudo permissions");
        exit(1);
    }
    let mut device = device.unwrap();

    let timeline: Timeline = rec.event_list.into();

    if gen.sequence {
        replay_in_loop(&mut device, &timeline)?;
    } else {
        replay_timeline(&mut device, &timeline)?;
    }

    Ok(())
}
