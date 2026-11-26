use psu_toggle::SOCKET_NAME;


fn main() -> std::io::Result<()> {
    //}
    use {
        interprocess::local_socket::{prelude::*, GenericNamespaced, ListenerOptions},
        std::io::{self, prelude::*, BufReader},
    };

    let name = SOCKET_NAME.to_ns_name::<GenericNamespaced>()?;

    let listener = match ListenerOptions::new().name(name).create_sync() {
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

    // This is a good place to inform clients that the server is ready.
    eprintln!("Server running at {SOCKET_NAME}");

    // This buffer will be reused between clients.
    let mut buffer = String::with_capacity(128);

    for mut conn in listener
        .incoming()
        .filter_map(|conn| conn.map_err(|e| eprintln!("Incoming connection failed: {e}")).ok())
        .map(BufReader::new)
    {
        // Since our client example sends first, the server should receive a
        // line and only then send a response. Otherwise, because receiving
        // from and sending to a connection cannot be simultaneous without
        // threads or async, we can deadlock the two processes by having both
        // sides wait for the send buffer to be emptied by the other.
        conn.read_line(&mut buffer)?;

        // Now that the receive has come through and the client is waiting
        // on the server's send, do it. (`.get_mut()` is to get the sender,
        // `BufReader` doesn't implement a pass-through `Write`.)
        conn.get_mut().write_all(b"Hello from server!\n")?;

        // Avoid holding up resources.
        drop(conn);

        // read_line keeps the line feed at the end.
        print!("Client answered: {buffer}");

        // Clear the buffer so that the next iteration will display new data
        // instead of messages stacking on top of one another.
        buffer.clear();
    }
    //{
    Ok(())
} //}
