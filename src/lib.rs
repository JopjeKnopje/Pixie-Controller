use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "example.sock";


#[derive(Serialize, Deserialize, Debug)]
pub enum PowerState {
    On,
    Off,
    Toggle,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Power(PowerState),
    Status,
}


