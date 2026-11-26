// #![deny(warnings)]
use anyhow::Context;
use ka3005p::{Command, Ka3005p, Switch};
use std::convert::TryInto;
use std::process::exit;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    cmd: String,
}

fn status(state: Switch) {

    let s = match state {
        Switch::On => "\u{f111}",
        Switch::Off => "\u{f10c}",
    };
    println!("{}", s);

}

fn output_set(mut serial: Ka3005p, state: Switch) ->  ::anyhow::Result<(), anyhow::Error> {
    let cmd = Command::Power(state);

    serial.execute(
        cmd.try_into()
            .with_context(|| "unsupported command conversion")?,
    )?;
    Ok(())
}

fn toggle(serial: Ka3005p, state: Switch) -> ::anyhow::Result<(), anyhow::Error> {
    let state = match state {
        Switch::On => Switch::Off,
        Switch::Off => Switch::On,
    };
    return output_set(serial, state)
}

fn main() -> ::anyhow::Result<(), anyhow::Error> {
    human_panic::setup_panic!();
    env_logger::init();

    // real faken dirty
    let cmd = Args::parse().cmd;

    let mut serial = ka3005p::find_serial_port()?;

    let state = serial.status()?.flags.output;



    match cmd.as_str() {
        "status" => status(state),
        "toggle" => toggle(serial, state)?,
        "on" => output_set(serial, Switch::On)?,
        "off" => output_set(serial, Switch::Off)?,
        _ => exit(1)
        
    }
    exit(0);
}

