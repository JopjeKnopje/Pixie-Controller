use anyhow::{Ok, Result};
use ka3005p::{Command as PsuCommand, Ka3005p};
use serde::{Deserialize, Serialize};

pub const SOCKET_NAME: &str = "example.sock";

type FnCustomCommand = fn(&mut Ka3005p) -> Result<Option<String>>;

// data we SEND to the power supply
#[derive(Serialize, Deserialize, Debug)]
pub enum CustomCommand {
    PowerToggle,
    PowerStatus,
}

impl std::convert::From<&str> for CustomCommand {
    fn from(c: &str) -> Self {
        match c {
            "toggle" => CustomCommand::PowerToggle,
            "status" => CustomCommand::PowerStatus,
            &_ => todo!(),
        }
    }
}

// lil bit of unreadable code hehe (I'm sorry)
fn fc_power_toggle(psu: &mut Ka3005p) -> Result<Option<String>> {
    let state: bool = psu.status()?.flags.output.into();
    psu.execute(PsuCommand::Power((!state).into()))?;
    Ok(None)
}

fn fc_power_status(psu: &mut Ka3005p) -> Result<Option<String>> {
    let state: bool = psu.status()?.flags.output.into();
    let s = match state {
        true => String::from("\u{f111}"),
        false => String::from("\u{f10c}"),
    };
    Ok(Some(s))
}

impl CustomCommand {
    pub fn enum_to_func(&self) -> FnCustomCommand {
        match self {
            CustomCommand::PowerToggle => fc_power_toggle,
            CustomCommand::PowerStatus => fc_power_status,
        }
    }
}
