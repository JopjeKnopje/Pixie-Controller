use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "example.sock";

#[derive(Serialize, Deserialize, Debug)]
pub enum PowerState {
    On,
    Off,
    Toggle,
}

// data we SEND to the power supply
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Power(PowerState),
}
