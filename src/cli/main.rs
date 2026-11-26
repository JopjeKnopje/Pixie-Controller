use std::{
    env,
    io::{BufRead, Write},
};

use psu_toggle::{CustomCommand, SOCKET_NAME};
use {
    interprocess::local_socket::{GenericNamespaced, Stream, prelude::*},
    std::io::BufReader,
};

fn main() -> std::io::Result<()> {
    human_panic::setup_panic!();
    let args: Vec<String> = env::args().collect();

    // yikes
    let cmd = CustomCommand::from(args[1].as_str());

    let name = SOCKET_NAME.to_ns_name::<GenericNamespaced>()?;

    // Will fail immediately if the server hasn't started yet.
    let mut conn = BufReader::new(Stream::connect(name)?);

    // 1. parse user input into `Command` enum.
    // 2. serialize and send enum through socket.
    // 3. print reply which will be a `ka3005p::Status` and parse that out

    let data = serde_cbor::to_vec(&cmd).unwrap();

    conn.get_mut().write_all(&data)?;

    // BufReader doesn't pass Write through, so we use get_mut.
    // conn.get_mut().write_all(b"Hello from client!\n")?;

    // We now employ the buffer we allocated prior and receive a single line,
    // interpreting a newline character as an end-of-file (because local
    // sockets cannot be portably shut down), verifying validity of UTF-8 on
    // the fly.
    let mut buffer = String::with_capacity(128);
    conn.read_line(&mut buffer)?;

    // Avoid holding up resources.
    drop(conn);

    // read_line keeps the line feed at the end.
    if !buffer.is_empty() {
        print!("{}", buffer);
    }
    //{
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_serialization() {
        let cmd = CustomCommand::PowerToggle;
        let data = serde_cbor::to_vec(&cmd).unwrap();
        println!("serialized data {:?}", data);

        let cmd_new = serde_cbor::from_slice::<CustomCommand>(data.as_slice()).unwrap();
        println!("de-serialized data {:?}", cmd_new);
    }
}
