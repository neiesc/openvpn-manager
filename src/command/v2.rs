use std::io;
use std::io::Write;
use crate::{
    cli::{Args, Command},
};
use std::process::{Command as SysCmd, Stdio};

pub fn handle(args: &Args) -> anyhow::Result<()> {
    match args.command {
        Command::Start => start(&args.vpn_config)?,
        Command::Status => status(&args.vpn_config)?,
        _ => todo!()
    }
    Ok(())
}

fn start(config: &str) -> anyhow::Result<()> {
    println!("Starting OpenVPN 2 with {}", config);
    print!("Username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim_end();
    let password = rpassword::prompt_password("Password: ")?;
    let password = password.trim_end();

    let fifo = "/tmp/openvpn_auth.fifo";
    let _ = std::fs::remove_file(fifo);
    let status_mk = SysCmd::new("mkfifo").arg(fifo).status()?;
    assert!(status_mk.success(), "mkfifo falhou");

    let writer = std::thread::spawn({
        let user = username.to_string();
        let pass = password.to_string();
        move || -> io::Result<()> {
            let mut f = std::fs::OpenOptions::new().write(true).open(fifo)?;
            writeln!(f, "{user}")?;
            writeln!(f, "{pass}")?;
            Ok(())
        }
    });

    let _status = SysCmd::new("sudo")
        .arg("openvpn")
        .arg("--config").arg(config)
        .arg("--auth-user-pass").arg(fifo)
        .stdin(Stdio::null())
        .status()?;

    let _ = writer.join();
    let _ = std::fs::remove_file(fifo);

    println!("OpenVPN 2 started.\nNote: if your config doesn’t use auth-user-pass, credentials may be ignored.");
    Ok(())
}

fn status(_config: &str) -> anyhow::Result<()> {
    let out = SysCmd::new("pgrep")
        .arg("openvpn")
        .output()?;
    if out.status.success() {
        println!("✅ openvpn appears to be running.");
    } else {
        println!("❌ no openvpn process found.");
    }
    Ok(())
}
