use anyhow::{Error, Result};
use interprocess::local_socket::Stream;
use ka3005p::Ka3005p;
use psu_toggle::{CustomCommand, SOCKET_NAME};

use {
    interprocess::local_socket::{GenericNamespaced, ListenerOptions, prelude::*},
    std::io::{self, BufReader, Read, prelude::*},
};

const BUFFER_SIZE: usize = 8;

fn ipc_init(socket_name: &str) -> Result<LocalSocketListener, std::io::Error> {
    let n = socket_name.to_ns_name::<GenericNamespaced>()?;
    let listener = match ListenerOptions::new().name(n).create_sync() {
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            eprintln!(
                "Error: could not start server because the socket file is \
                occupied. Please check if {SOCKET_NAME} is in use by another \
                process and try again."
            );
            return Err(e);
        }
        x => x?,
    };
    Ok(listener)
}

// TODO: clean up this code, make it more "rusty"
fn ipc_receive(conn: &mut BufReader<Stream>) -> Result<CustomCommand, Error> {
    let mut data = Vec::<u8>::new();
    loop {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        let n = conn.read(&mut buffer)?;
        if n > 0 {
            for b in buffer {
                if b > 0 {
                    data.push(b);
                }
            }
        }
        if n < BUFFER_SIZE {
            break;
        }
    }
    Ok(serde_cbor::from_slice::<CustomCommand>(data.as_slice())?)
}

fn psu_init() -> Result<Ka3005p, Error> {
    // TODO: Print the serial port.
    // TODO: Fix the redant wrapping of the return values
    match ka3005p::find_serial_port() {
        Ok(dev) => {
            println!("connected to psu on serial port");
            Ok(dev)
        }
        Err(e) => {
            eprintln!("failed connecting with the power supply {}", e);
            Err(e)
        }
    }
}

fn main() -> Result<()> {
    human_panic::setup_panic!();

    let mut psu = psu_init()?;

    let listener = ipc_init(SOCKET_NAME)?;
    eprintln!("Server running at {SOCKET_NAME}");
    for mut conn in listener
        .incoming()
        .filter_map(|conn| {
            conn.map_err(|e| eprintln!("Incoming connection failed: {e}"))
                .ok()
        })
        .map(BufReader::new)
    {
        let cmd = ipc_receive(&mut conn).unwrap();
        println!("received command {:?}", cmd);

        let func = cmd.enum_to_func();
        func(&mut psu)?;

        conn.get_mut().write_all(b"Hello from server!\n")?;

        // Avoid holding up resources.
        drop(conn);
    }
    Ok(())
}
