use anyhow::{Ok, Result};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, AttributeSetRef, Device, EvdevEnum, EventType, InputEvent, Key,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub device: DeviceDescriptor,
    pub event_list: Vec<EventDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDescriptor {
    time: u128,
    event_type: u16,
    code: u16,
    value: i32,
}

impl EventDescriptor {
    pub fn new(time: u128, event_type: u16, code: u16, value: i32) -> Self {
        EventDescriptor {
            time,
            event_type,
            code,
            value,
        }
    }
}

pub struct Keyframe {
    pub time: u128,
    pub events: Vec<InputEvent>,
}

pub struct Timeline {
    pub keyframes: Vec<Keyframe>,
}

impl From<Vec<EventDescriptor>> for Timeline {
    fn from(events: Vec<EventDescriptor>) -> Self {
        let mut t = Timeline {
            keyframes: Vec::new(),
        };

        for ev in events {
            let mut k = Keyframe {
                time: ev.time,
                events: Vec::new(),
            };
            k.events
                .push(InputEvent::new(EventType(ev.event_type), ev.code, ev.value));

            t.keyframes.push(k);
        }

        t
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    events: Vec<u16>,
    keys: Option<Vec<u16>>,
    relative_axes: Option<Vec<u16>>,
    absolute_axes: Option<Vec<u16>>,
    switches: Option<Vec<u16>>,
    leds: Option<Vec<u16>>,
    sounds: Option<Vec<u16>>,
    ff: Option<Vec<u16>>,
}

pub fn create_device_descriptor(keys: Vec<Key>) -> Result<DeviceDescriptor> {
    let events = vec![0, 1, 2, 3, 4, 17, 20];
    let k: Vec<u16> = keys.iter().map(|el| el.0).collect();
    let desc = DeviceDescriptor {
        events,
        keys: Some(k),
        relative_axes: None,
        absolute_axes: None,
        switches: None,
        leds: None,
        sounds: None,
        ff: None,
    };

    Ok(desc)
}

fn flatten_attr_set<T>(set: Option<&AttributeSetRef<T>>) -> Option<Vec<u16>>
where
    T: EvdevEnum,
{
    let d: Vec<u16> = set.unwrap().iter().map(|t| t.to_index() as u16).collect();
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

impl From<&Device> for DeviceDescriptor {
    fn from(dev: &Device) -> Self {
        let supported_events: Vec<u16> = dev.supported_events().iter().map(|ev| ev.0).collect();

        let mut disc = DeviceDescriptor {
            events: supported_events,
            keys: None,
            relative_axes: None,
            absolute_axes: None,
            switches: None,
            leds: None,
            sounds: None,
            ff: None,
        };

        for event in dev.supported_events().iter() {
            match event {
                EventType::KEY => disc.keys = flatten_attr_set(dev.supported_keys()),
                EventType::RELATIVE => {
                    disc.relative_axes = flatten_attr_set(dev.supported_relative_axes())
                }
                EventType::ABSOLUTE => {
                    disc.absolute_axes = flatten_attr_set(dev.supported_absolute_axes())
                }
                EventType::SWITCH => {
                    disc.absolute_axes = flatten_attr_set(dev.supported_switches())
                }
                EventType::LED => disc.absolute_axes = flatten_attr_set(dev.supported_leds()),
                EventType::SOUND => disc.absolute_axes = flatten_attr_set(dev.supported_sounds()),
                EventType::FORCEFEEDBACK => disc.ff = flatten_attr_set(dev.supported_ff()),
                _ => (),
            }
        }

        disc
    }
}

impl TryFrom<DeviceDescriptor> for VirtualDevice {
    type Error = std::io::Error;

    fn try_from(disc: DeviceDescriptor) -> Result<Self, Self::Error> {
        let mut keys = AttributeSet::<Key>::new();
        if disc.keys.is_some() {
            for key in disc.keys.unwrap() {
                keys.insert(Key::new(key));
            }
        }

        VirtualDeviceBuilder::new()?
            .name("fake device")
            .with_keys(&keys)?
            .build()
    }
}
