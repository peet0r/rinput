use anyhow::Result;
use clap::Parser;
use evdev::uinput::VirtualDevice;
use std::result::Result::Ok;
use std::{fs::File, process::exit};
mod cli;
mod descriptor;
mod record;
mod replay;

use crate::descriptor::{Recording, Timeline};
use crate::record::start_recording;
use crate::replay::{replay_in_loop, replay_timeline};
use cli::{pick_device, Cli, RInputCommand, Record, Replay};
use record::get_devices;

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("WARNING: This tool currently only works for key events");
    if (match args.command {
        RInputCommand::Record(rec) => process_record(rec),
        RInputCommand::Replay(rep) => process_replay(rep),
    })
    .is_ok()
    {
        println!("Finished: OK");
    } else {
        println!("Finished: Error")
    }

    Ok(())
}

fn process_record(rec: Record) -> Result<(), anyhow::Error> {
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

fn process_replay(rep: Replay) -> Result<(), anyhow::Error> {
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
