use anyhow::{Ok, Result};
use clap::Parser;
use descriptor::{create_device_descriptor, EventDescriptor};
use evdev::uinput::VirtualDevice;
use generate::parse_file;
use std::{fs::File, process::exit};
mod cli;
mod descriptor;
mod generate;
mod record;
mod replay;
mod utils;

use crate::descriptor::{Recording, Timeline};
use crate::record::start_recording;
use crate::replay::{replay_in_loop, replay_timeline};
use cli::{pick_device, Cli, Generate, RInputCommand, Record, Replay};
use record::{get_devices, write_to_file};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    println!("WARNING: This tool currently only works for key events");
    if (match args.command {
        RInputCommand::Record(rec) => process_record(rec).await,
        RInputCommand::Replay(rep) => process_replay(rep).await,
        RInputCommand::Generate(gen) => process_generate(gen).await,
    })
    .is_ok()
    {
        println!("Finished: OK");
    } else {
        println!("Finished: Error")
    }

    Ok(())
}

async fn process_record(rec: Record) -> Result<(), anyhow::Error> {
    if rec.enumerate {
        println!("run enumerate and exit");
        for device in get_devices().iter() {
            println!("{:?}", device);
        }
        exit(0);
    }

    // Check that output file path is valid
    if rec.output.is_none() {
        println!("Output Path is not provided");
        //TODO: this is a crappy error
        exit(1);
    }

    let f = File::create(rec.output.unwrap().as_str())?;

    // Get desired device from user
    let devices = get_devices();
    let device_path = pick_device(devices);
    println!("Initializing device at: {:?}", device_path);

    start_recording(device_path, f)?;

    Ok(())
}

async fn process_replay(rep: Replay) -> Result<(), anyhow::Error> {
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

    for t in keys.iter().enumerate() {
        let delta = gen.delta * (t.0 as u64 + 2);
        let key = t.1;
        //Down
        rec.event_list
            .push(EventDescriptor::new(delta.into(), 1, key.0, 1));
        //Up
        rec.event_list.push(EventDescriptor::new(
            (delta as f64 * 1.1).ceil() as u128,
            1,
            key.0,
            0,
        ));
    }

    if gen.output.is_some() {
        let file = File::create(gen.output.unwrap())?;
        write_to_file(rec.clone(), file)?;
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
