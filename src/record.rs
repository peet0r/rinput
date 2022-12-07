use crate::descriptor::{DeviceDescriptor, EventDescriptor, Recording};
use anyhow::Result;
use console::Term;
use evdev::{enumerate, Device, InputEvent, InputEventKind, Key};
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
