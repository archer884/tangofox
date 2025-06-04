mod addr;
mod timeout;

use std::{env, io, path::PathBuf, time::Duration};

use clap::Parser;
use libunftp::options::Shutdown;
use unftp_sbe_fs::ServerExt;

use crate::{addr::Address, timeout::Timeout};

type Server = libunftp::Server<unftp_sbe_fs::Filesystem, libunftp::auth::DefaultUser>;

#[derive(Debug, clap::Parser)]
#[command(version, about)]
struct Args {
    path: Option<String>,

    // FIXME My gut tells me we should have a default timeout, but I'm annoyed by the idea...
    #[arg(short, long)] //, default_value_t = Timeout::Minutes(5))]
    timeout: Option<Timeout>, // Timeout,

    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 2121)]
    port: u16,
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

    fn address(&self) -> Address {
        Address::new(self.port)
    }
}

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    if let Err(e) = run(Args::parse()).await {
        eprintln!("{e}");
    }
}

async fn run(args: Args) -> io::Result<()> {
    let mut server = libunftp::Server::with_fs(args.root()?)
        .greeting("Leave your files at the door and get the hell out of here.")
        .passive_ports(50000..=65535);

    if let Some(t) = args.timeout {
        server = server.shutdown_indicator(wait_timeout(t.duration()));
    }

    println!("Ftp running on port {}...", args.port);
    listen(server.build().unwrap(), args.address()).await?;

    Ok(())
}

async fn listen(server: Server, addr: Address) -> io::Result<()> {
    server.listen(addr).await.map_err(io::Error::other)
}

async fn wait_timeout(duration: Duration) -> Shutdown {
    tokio::time::sleep(duration).await;
    Shutdown::default().grace_period(Duration::from_secs(5))
}
