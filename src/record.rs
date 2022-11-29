use crate::descriptor::{DeviceDescriptor, EventDescriptor, Recording};
use anyhow::Result;
use console::Term;
use evdev::{enumerate, Device, EventType, InputEventKind, Key};
use std::process;
use std::{fs::File, time::Instant};

pub fn get_devices() -> Vec<(String, String)> {
    let mut devices = enumerate()
        .map(|t| {
            (
                t.0.to_str().unwrap().to_string(),
                t.1.name().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();
    devices.reverse();
    devices
}

pub fn start_recording(device_path: String, outputfile: File) -> Result<()> {
    let term = Term::stdout();

    // Create device
    let mut device = Device::open(&device_path)?;

    // Setup file writer
    // Write header to file
    let mut j = Recording::new(DeviceDescriptor::from(&device));

    // Record Event Time
    let init_time = Instant::now();

    let mut esc_pressed = false;
    let mut lctrl_pressed = false;
    term.write_line("Recording events from device, press ESC and Left-CTRL to save and exit")?;
    term.write_line(&format!(
        "left-ctrl: {} esc: {}",
        lctrl_pressed, esc_pressed
    ))?;
    // Loop
    loop {
        for event in device.fetch_events()? {
            let event_timestamp = init_time.elapsed();
            // Parse event into relavent data
            match event.event_type() {
                EventType::KEY => {
                    if event.kind() == InputEventKind::Key(Key::KEY_ESC) {
                        if event.value() == 0 {
                            esc_pressed = false;
                        } else {
                            esc_pressed = true;
                        }
                    }

                    if event.kind() == InputEventKind::Key(Key::KEY_LEFTCTRL) {
                        if event.value() == 0 {
                            lctrl_pressed = false;
                        } else {
                            lctrl_pressed = true;
                        }
                    }
                    term.clear_last_lines(1)?;
                    term.write_line(&format!(
                        "left-ctrl: {} esc: {}",
                        lctrl_pressed, esc_pressed
                    ))?;
                    if lctrl_pressed && esc_pressed {
                        write_to_file(j, outputfile)?;
                        process::exit(1);
                    }

                    j.event_list.push(EventDescriptor::new(
                        event_timestamp.as_millis(),
                        event.event_type().0,
                        event.code(),
                        event.value(),
                    ));
                }
                _ => (),
            }
            // Append to file
        }
    }
}

pub fn write_to_file(rec: Recording, outputfile: File) -> Result<()> {
    serde_json::to_writer(outputfile, &rec)?;
    Ok(())
}
