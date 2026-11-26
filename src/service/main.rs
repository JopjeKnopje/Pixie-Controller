use psu_toggle::SOCKET_NAME;
use serialport::new;

fn main() -> std::io::Result<()> {
    //}
    use {
        interprocess::local_socket::{GenericNamespaced, ListenerOptions, prelude::*},
        std::io::{self, BufReader, prelude::*},
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
    const BUFFER_SIZE: usize = 8;

    for mut conn in listener
        .incoming()
        .filter_map(|conn| {
            conn.map_err(|e| eprintln!("Incoming connection failed: {e}"))
                .ok()
        })
        .map(BufReader::new)
    {
        // Since our client example sends first, the server should receive a
        // line and only then send a response. Otherwise, because receiving
        // from and sending to a connection cannot be simultaneous without
        // threads or async, we can deadlock the two processes by having both
        // sides wait for the send buffer to be emptied by the other.
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
            dbg!("received {:?}", &data);

            if n < BUFFER_SIZE {
                break;
            }
        }
        // Now that the receive has come through and the client is waiting
        // on the server's send, do it. (`.get_mut()` is to get the sender,
        // `BufReader` doesn't implement a pass-through `Write`.)
        conn.get_mut().write_all(b"Hello from server!\n")?;

        // Avoid holding up resources.
        drop(conn);

        // read_line keeps the line feed at the end.
        print!("Client answered: {data:?}");

        // Clear the buffer so that the next iteration will display new data
        // instead of messages stacking on top of one another.
        // buffer.clear();
    }
    //{
    Ok(())
} //}
