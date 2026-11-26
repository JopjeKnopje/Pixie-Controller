use anyhow::Result;
use ka3005p::{Command as PsuCommand, Ka3005p};
use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "example.sock";

type FnCustomCommand = fn(&mut Ka3005p) -> Result<()>;

// data we SEND to the power supply
#[derive(Serialize, Deserialize, Debug)]
pub enum CustomCommand {
    PowerToggle,
}

// lil bit of unreadable code hehe (I'm sorry)
fn fc_power_toggle(psu: &mut Ka3005p) -> Result<()> {
    let state: bool = psu.status()?.flags.output.into();
    psu.execute(PsuCommand::Power((!state).into()))
}

impl CustomCommand {
    pub fn enum_to_func(&self) -> FnCustomCommand {
        match self {
            CustomCommand::PowerToggle => fc_power_toggle,
        }
    }
}
