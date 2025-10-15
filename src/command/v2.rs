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

    let mut child = SysCmd::new("openvpn")
        .args(["--config", config])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "auth-user-pass")?;
        writeln!(stdin, "{}", username)?;
        writeln!(stdin, "{}", password)?;
    }

    println!("OpenVPN 2 started (background).\nNote: if your config doesn’t use auth-user-pass, credentials may be ignored.");
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
