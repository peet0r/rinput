use std::time::Instant;

use anyhow::{Ok, Result};
use console::Term;
use evdev::uinput::VirtualDevice;

use crate::descriptor::Timeline;

pub fn start_replaying(mut device: VirtualDevice, timeline: Timeline) -> Result<()> {
    let term = Term::stdout();

    let values = timeline.keyframes;
    let mut i = 0;
    let mut current = &values[i];

    term.write_line("Running")?;
    term.write_line("Press any key to fire events, q to quit")?;
    loop {
        term.write_line("Sending sequence")?;
        let v = term.read_char()?;
        if v == 'q' {
            break;
        }
        let start = Instant::now();
        i = 0;
        loop {
            let t = start.elapsed().as_micros();
            if t > current.time {
                current = &values[i];
                device.emit(&current.events)?;
                i += 1;
                if i >= values.len() - 1 {
                    break;
                }
            }
        }
    }

    Ok(())
}
