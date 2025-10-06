mod cli;
mod util;
mod vpn;

use clap::Parser;
use cli::{Args, OpenVpnVersion};
use vpn::{v2, v3};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.version {
        OpenVpnVersion::V2 => v2::handle(&args)?,
        OpenVpnVersion::V3 => v3::handle(&args)?,
    }

    Ok(())
}
