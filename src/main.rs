use std::io::{self, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Command;
use std::thread;

const MAGIC_PACKET: u8 = 0x77;

enum WakeState {
    Sleep,
    Unknown(u8),
}

impl From<u8> for WakeState {
    fn from(value: u8) -> Self {
        match value {
            MAGIC_PACKET => WakeState::Sleep,
            _ => WakeState::Unknown(value),
        }
    }
}

fn process(mut socket: TcpStream) -> io::Result<()> {
    let mut packet = [0];
    socket.read_exact(&mut packet)?;
    let state: WakeState = packet[0].into();
    match state {
        WakeState::Sleep => {
            eprintln!("Putting the server to sleep");
            Command::new("systemctl").arg("suspend").output()?;
            eprintln!("Server is now awake");
        }
        WakeState::Unknown(value) => {
            eprintln!("Unknown command: {value:#04x}");
        }
    };

    Ok(())
}

fn main() -> io::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8253));
    let listener = TcpListener::bind(addr)?;
    eprintln!("Listening on port: {}", addr.port());
    loop {
        let (socket, _) = listener.accept()?;
        thread::spawn(move || {
            eprintln!("Got a connection");
            if let Err(error) = process(socket) {
                eprintln!("Connection error: {error}");
            }
        });
    }
}
