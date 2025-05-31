use std::{env, io, path::PathBuf};

use clap::Parser;
use unftp_sbe_fs::ServerExt;

#[derive(Debug, clap::Parser)]
struct Args {
    path: Option<String>,
}

impl Args {
    fn root(&self) -> io::Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.into()),
            None => env::current_dir().map_err(|_| {
                io::Error::new(io::ErrorKind::NotFound, "failed to get working directory")
            }),
        }
    }
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let args = Args::parse();
    let root = args.root()?;

    let server = libunftp::Server::with_fs(root)
        .greeting("Leave your files at the door and get the hell out of here.")
        .passive_ports(50000..=65535)
        .build()
        .unwrap();

    server
        .listen("0.0.0.0:2121")
        .await
        .map_err(io::Error::other)?;

    Ok(())
}
