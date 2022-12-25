use std::time::Instant;

use anyhow::{Ok, Result};
use console::Term;
use evdev::uinput::VirtualDevice;

use crate::descriptor::Timeline;

pub fn replay_timeline(device: &mut VirtualDevice, timeline: &Timeline) -> Result<()> {
    let values = &timeline.keyframes;
    let start = Instant::now();

    for frame in values{
        while start.elapsed().as_micros() > frame.time{}
        device.emit(&frame.events)?;
    }

    Ok(())
}

pub fn replay_in_loop(device: &mut VirtualDevice, timeline: &Timeline) -> Result<()> {
    let term = Term::stdout();

    term.write_line("Running")?;
    term.write_line("Press any key to fire events, q to quit")?;
    loop {
        term.write_line("Sending sequence")?;
        let v = term.read_char()?;
        if v == 'q' {
            break;
        }
        replay_timeline(device, timeline)?
    }

    Ok(())
}
