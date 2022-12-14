use crate::cli::pick_device;
use crate::descriptor::{DeviceDescriptor, EventDescriptor, Recording};
use anyhow::{anyhow, Result};
use console::Term;
use evdev::{enumerate, Device};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

use std::time::SystemTime;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum Msg {
    Exit,
    Event(EventDescriptor),
    Device(DeviceDescriptor),
}

// Returns (path, device name)
pub fn get_devices() -> Result<Vec<(String, String)>> {
    let mut devices = enumerate()
        .map(|t| {
            (
                t.0.to_str().unwrap().to_string(),
                t.1.name().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();

    if devices.is_empty() {
        return Err(anyhow!("No devices detected"));
    }

    devices.reverse();
    Ok(devices)
}

// List devices
pub fn enumerate_devices() -> Result<()> {
    let devices = get_devices()?;

    for (_, dev) in devices {
        println!("{}", dev);
    }
    Ok(())
}

pub fn select_device() -> Result<String> {
    let devices = get_devices()?;

    pick_device(devices)
}

pub fn validate_path(output: String) -> Result<PathBuf> {
    let path = Path::new(&output);

    if path.exists() {
        return Err(anyhow!("output file already exists"));
    }

    Ok(path.to_path_buf())
}

pub async fn listen_loop(tx: Sender<Msg>, device_path: String) -> Result<()> {
    let term = Term::stdout();

    // Create device
    let device = Device::open(&device_path)?;
    tx.send(Msg::Device(DeviceDescriptor::from(&device)))
        .await?;

    // Record Event Time
    let now = SystemTime::now();

    let mut event_index: u64 = 1;

    let mut events = device.into_event_stream()?;
    term.write_line("")?;
    loop {
        let ev = events.next_event().await?;

        let desc = EventDescriptor::new(
            ev.timestamp().duration_since(now)?.as_millis(),
            ev.event_type().0,
            ev.code(),
            ev.value(),
        );

        tx.send(Msg::Event(desc.clone())).await?;
        if ev.event_type().0 != 0 {
            term.clear_last_lines(1)?;
            term.write_line(&format!("{} Event: {:?}", event_index, desc))?;
        }
        event_index += 1;
    }
}

pub async fn record_loop(mut rx: Receiver<Msg>, output: PathBuf) -> Result<()> {
    let term = Term::stdout();
    let mut device: Option<DeviceDescriptor> = None;
    let mut events: Vec<EventDescriptor> = Vec::new();

    // Make a File
    while let Some(msg) = rx.recv().await {
        match msg {
            Msg::Exit => {
                term.clear_last_lines(1)?;
                term.write_line("Exit Command Received")?;
                let rec = Recording {
                    event_list: events,
                    device: device.unwrap(),
                };
                write_to_file(output, rec).await?;
                return Ok(());
            }
            Msg::Event(value) => {
                // If verbose?
                events.push(value);
            }
            Msg::Device(dev) => {
                device = Some(dev);
            }
        }
    }

    Ok(())
}

pub async fn write_to_file(output: PathBuf, rec: Recording) -> Result<()> {
    let term = Term::stdout();
    term.write_line(format!("Writing to file: {:?}", output).as_str())?;
    let json = serde_json::to_string(&rec)?;

    let mut file = tokio::fs::File::create(output).await?;

    file.write_all(json.as_bytes()).await?;
    Ok(())
}
