
use std::io::{BufRead, Write};

use psu_toggle::{Command, PowerState::Toggle, SOCKET_NAME};
use serde::Serialize;
use {
    interprocess::local_socket::{GenericNamespaced, Stream, prelude::*},
    std::io::BufReader,
};

fn main() -> std::io::Result<()> {
    let name = SOCKET_NAME.to_ns_name::<GenericNamespaced>()?;
    let mut buffer = String::with_capacity(128);

    // Will fail immediately if the server hasn't started yet.
    let mut conn = BufReader::new(Stream::connect(name)?);

    // BufReader doesn't pass Write through, so we use get_mut.
    conn.get_mut().write_all(b"Hello from client!\n")?;

    // We now employ the buffer we allocated prior and receive a single line,
    // interpreting a newline character as an end-of-file (because local
    // sockets cannot be portably shut down), verifying validity of UTF-8 on
    // the fly.
    conn.read_line(&mut buffer)?;

    // Avoid holding up resources.
    drop(conn);

    // read_line keeps the line feed at the end.
    print!("Server answered: {buffer}");
    //{
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_data() {
        let cmd = Command::Power(Toggle);
        let data = serde_cbor::to_vec(&cmd).unwrap();
        println!("serialized data {:?}", data);

        let cmd_new = serde_cbor::from_slice::<Command>(data.as_slice()).unwrap();
        println!("de-serialized data {:?}", cmd_new);
    }
}
