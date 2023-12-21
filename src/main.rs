use std::net::SocketAddr;
use std::process::Command;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

#[repr(u8)]
enum WakeState {
    Sleep,
    Unknown(u8),
}

impl From<u8> for WakeState {
    fn from(value: u8) -> Self {
        match value {
            0x01 => WakeState::Sleep,
            _ => WakeState::Unknown(value),
        }
    }
}

async fn process(mut socket: TcpStream) -> anyhow::Result<()> {
    let state: WakeState = socket.read_u8().await?.into();
    match state {
        WakeState::Sleep => {
            info!("Putting the server to sleep");
            Command::new("systemctl").arg("suspend").output()?;
            info!("Server is now awake");
        }
        WakeState::Unknown(value) => {
            error!("Unknown command: {:#04x}", value);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on port: {}", addr.port());
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            info!("Got a connection");
            let _ = process(socket).await;
        });
    }
}
