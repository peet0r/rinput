use crate::cli::pick_device;
use crate::descriptor::{DeviceDescriptor, EventDescriptor, Recording};
use anyhow::{anyhow, Result};
use console::Term;
use evdev::{enumerate, Device, InputEvent, InputEventKind, Key};
use libc::exit;
use std::path::{Path, PathBuf};
use std::process;
use std::time::SystemTime;
use std::{fs::File, time::Instant};
use tokio::sync::mpsc::{Receiver, Sender};

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

    if devices.len() == 0 {
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

    return pick_device(devices);
}

pub fn validate_path(output: String) -> Result<PathBuf> {
    let path = Path::new(&output);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(anyhow!("output path is not valid"));
        }
    }

    if path.exists() == true {
        return Err(anyhow!("output file already exists"));
    }

    Ok(path.to_path_buf())
}

pub async fn listen_loop(tx: Sender<i64>, device_path: String) -> Result<()> {
    let term = Term::stdout();

    // Create device
    let device = Device::open(&device_path)?;

    // Record Event Time
    let now = SystemTime::now();

    let mut event_index: i64 = 1;
    // TODO: Validate that this is correct Tokio async stream for events
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

        tx.send(event_index).await?;
        term.clear_last_lines(1)?;
        term.write_line(&format!("{} Event: {:?}", event_index, desc))?;
        event_index += 1;
    }
}

pub async fn record_loop(mut rx: Receiver<i64>, output: PathBuf) -> Result<()> {
    let term = Term::stdout();

    // Make a File
    while let Some(i) = rx.recv().await {
        match i {
            0 => {
                term.write_line("Exit")?;
            }
            _ => continue,
        }
        // Wait for events and write them to file...?
    }

    Ok(())
}

pub fn start_recording(device_path: String, outputfile: File) -> Result<()> {
    // Create device
    let mut device = Device::open(&device_path)?;

    // Setup file writer
    let mut j = Recording::new(DeviceDescriptor::from(&device));

    // Record Event Time
    let init_time = Instant::now();

    let mut controller = LoopController::new();

    // Loop
    loop {
        for event in device.fetch_events()? {
            let event_timestamp = init_time.elapsed();

            if controller.should_exit(event)? {
                write_to_file(j, outputfile)?;
                process::exit(1);
            }

            // Parse event into relavent data
            j.event_list.push(EventDescriptor::new(
                event_timestamp.as_millis(),
                event.event_type().0,
                event.code(),
                event.value(),
            ));
        }
    }
}

pub fn write_to_file(rec: Recording, outputfile: File) -> Result<()> {
    serde_json::to_writer(outputfile, &rec)?;
    Ok(())
}

struct LoopController {
    esc_keystate: bool,
    left_control_keystate: bool,
    term: Term,
}

impl LoopController {
    fn new() -> Self {
        let term = Term::stdout();

        term.write_line("Recording events from device, press ESC and Left-CTRL to save and exit")
            .unwrap();

        LoopController {
            esc_keystate: false,
            left_control_keystate: false,
            term,
        }
    }
    fn should_exit(&mut self, event: InputEvent) -> Result<bool> {
        if event.kind() == InputEventKind::Key(Key::KEY_ESC) {
            if event.value() == 0 {
                self.esc_keystate = false;
            } else {
                self.esc_keystate = true;
            }
        }

        if event.kind() == InputEventKind::Key(Key::KEY_LEFTCTRL) {
            if event.value() == 0 {
                self.left_control_keystate = false;
            } else {
                self.left_control_keystate = true;
            }
        }
        self.term.clear_last_lines(1)?;
        self.term.write_line(&format!(
            "esc: {} left-ctrl: {}",
            self.esc_keystate, self.left_control_keystate
        ))?;

        Ok(self.esc_keystate && self.left_control_keystate)
    }
}
