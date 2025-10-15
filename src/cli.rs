use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OpenVpnVersion {
    #[value(name = "2")]
    V2,
    #[value(name = "3")]
    V3,
}
#[derive(Clone, ValueEnum)]
pub enum Command {
    Start,
    Stop,
    Restart,
    Status,
    MyIp,
}

#[derive(Parser)]
#[command(author, version, about = "Wrapper for OpenVPN 2/3")]
pub struct Args {
    #[arg(value_enum)]
    pub command: Command,

    #[arg(value_enum)]
    pub open_vpn_version: OpenVpnVersion,

    pub vpn_config:  String,
}
