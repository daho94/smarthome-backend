pub extern crate palette;

use palette::{rgb::Srgb, Color, Component, Shade};
use serde_json::{json, Value};
use std::io::prelude::*;
use std::io::Error as IOError;
use std::net::TcpStream;
use std::time::Duration;

pub struct Hyperion {
    socket_adr: String,
    priority: usize,
}

impl Hyperion {
    pub fn new(socket_adr: &str) -> Self {
        Self {
            socket_adr: socket_adr.to_owned(),
            priority: 1,
        }
    }

    pub fn new_with_priority(socket_adr: &str, priority: usize) -> Self {
        Self {
            socket_adr: socket_adr.to_owned(),
            priority,
        }
    }

    // Change color
    pub fn set_color(&self, color: Color) -> Result<(), IOError> {
        // Convert Color to SRGB and converts color range into 0-255
        let srgb = Srgb::from(color).into_format::<u8>();

        // Prepare command
        let command = json!({
            "command": "color",
            "color": vec!(srgb.red, srgb.green, srgb.blue),
            "priority": self.priority
        });

        // Create tcp stream
        let mut stream = self.create_stream()?;

        // Write data to stream
        let _ = send_command(&command.to_string(), &mut stream);

        Ok(())
    }

    // Set effect
    pub fn set_effect(&self, effect: &str) -> Result<(), IOError> {
        let command = json!({
            "command": "effect",
            "effect": {
                "name": effect
            },
            "priority": self.priority
        });

        let mut stream = self.create_stream()?;
        let _ = send_command(&command.to_string(), &mut stream);

        Ok(())
    }

    // Get current serverinfo
    pub fn get_serverinfo(&self) -> Result<Value, IOError> {
        let mut stream = self.create_stream()?;

        let command = json!({
            "command": "serverinfo"
        });

        let _ = stream.set_read_timeout(Some(Duration::from_millis(100)));

        let _ = send_command(&command.to_string(), &mut stream);

        let mut buffer = String::new();
        let _ = stream.read_to_string(&mut buffer);

        Ok(serde_json::from_str(&buffer).expect("Could not parse serverinfo!"))
    }

    // Clear all current effects or colors
    pub fn clear_all(&self) -> Result<(), IOError> {
        let command = json!({
            "command": "clearall",
        });

        let mut stream = self.create_stream()?;

        let _ = send_command(&command.to_string(), &mut stream);

        Ok(())
    }

    // Lighten the color by amount
    pub fn lighten(&self, amount: f32) -> Result<(), IOError> {
        if let Some(srgb) = self.parse_color_components() {
            let lightened = srgb.into_linear().lighten(amount);
            self.set_color(lightened.into())
        } else {
            println!("No active color is set to be lightened.");
            Ok(())
        }
    }

    // Darken the color by amount
    pub fn darken(&self, amount: f32) -> Result<(), IOError> {
        if let Some(srgb) = self.parse_color_components() {
            let darkened = srgb.into_linear().darken(amount);
            self.set_color(darkened.into())
        } else {
            println!("No active color is set to be darkened.");
            Ok(())
        }
    }

    pub fn get_effects(&self) -> Option<Vec<String>> {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct Effect {
            name: String,
        }

        let server_info = self
            .get_serverinfo()
            .expect("Could not get current serverinfo");

        if let Ok(effects) =
            serde_json::from_str::<Vec<Effect>>(&server_info["info"]["effects"].to_string())
        {
            Some(effects.into_iter().map(|e| e.name).collect())
        } else {
            None
        }
    }

    fn parse_color_components(&self) -> Option<Srgb> {
        let server_info = self
            .get_serverinfo()
            .expect("Could not get current serverinfo");
        let rgb = &server_info["info"]["activeLedColor"][0]["RGB Value"];

        if *rgb == Value::Null {
            return None;
        }

        let red = rgb[0]
            .to_string()
            .parse::<u8>()
            .expect("Could not parse value for red");
        let green = rgb[1]
            .to_string()
            .parse::<u8>()
            .expect("Could not parse value for green");
        let blue = rgb[2]
            .to_string()
            .parse::<u8>()
            .expect("Could not parse value for blue");

        Some(Srgb::new(red.convert(), green.convert(), blue.convert()))
    }

    fn create_stream(&self) -> Result<TcpStream, IOError> {
        let stream = TcpStream::connect_timeout(
            &self.socket_adr.parse().unwrap(),
            Duration::from_millis(5000),
        )?;
        Ok(stream)
    }
}

fn send_command(data: &str, stream: &mut TcpStream) -> Result<usize, IOError> {
    // Append new-line character to mark EoF
    let write_data = format!("{}\n", data);

    let _ = stream.set_write_timeout(Some(Duration::from_millis(500)));
    stream.write(&write_data.into_bytes())
}
