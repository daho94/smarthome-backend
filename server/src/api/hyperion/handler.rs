#[derive(Debug, Deserialize)]
pub struct HyperionCmd {
    pub address: String,
    pub command: Command,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Color { r: u8, g: u8, b: u8 },
    Effect { name: String },
}

#[derive(Debug, Deserialize)]
pub struct HyperionCmdAddr {
    pub address: String,
}
