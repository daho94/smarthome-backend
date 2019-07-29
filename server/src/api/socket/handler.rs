use funksteckdose::{wiringpi::WiringPiPin, Device, EncodingA, Protocol1, State};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ControlArgs {
    pub group: String,
    pub device: String,
    pub state: String,
}

pub fn send_socket_state(args: &ControlArgs) {
    type Socket = funksteckdose::Funksteckdose<WiringPiPin, EncodingA, Protocol1>;

    let pin = WiringPiPin::new(0);
    let d: Socket = Socket::new(pin);

    let device = Device::from_str(&args.device).expect("Could not parse device");
    let state = State::from_str(&args.state).expect("Could not parse state");

    d.send(&args.group, &device, &state)
        .expect("Failed to send");
}
