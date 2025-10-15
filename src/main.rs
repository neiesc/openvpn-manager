mod cli;
mod util;
mod command;

use clap::Parser;
use cli::{Args, OpenVpnVersion, Command};
use command::{v2, v3, myip};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::MyIp => myip::handle()?,
        _ => match args.open_vpn_version {
            OpenVpnVersion::V2 => v2::handle(&args)?,
            OpenVpnVersion::V3 => v3::handle(&args)?,
        }
    };

    Ok(())
}
