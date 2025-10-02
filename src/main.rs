use std::env;
use std::io::{BufRead, BufReader};
use std::process::{exit, Command};

// Função para exibir a mensagem de uso e sair
fn print_usage_and_exit(program_name: &str) {
    eprintln!("Uso: {} 2|3 start|stop file.ovpn", program_name);
    exit(1);
}

// Função para executar um comando externo e verificar erros
fn execute_command(mut command: Command) -> Result<(), String> {
    eprintln!("Executando comando: {:?}", command);
    match command.status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(format!("Comando falhou com status: {}", status))
            }
        }
        Err(e) => Err(format!("Erro ao executar o comando: {}", e)),
    }
}

fn main() {
    // 1. Coleta e valida os argumentos
    let args: Vec<String> = env::args().collect();
    let program_name = args.get(0).map(|s| s.as_str()).unwrap_or("openvpn_switcher");

    if args.len() != 4 {
        print_usage_and_exit(program_name);
    }

    let version_openvpn = &args[1];
    let command_arg = &args[2];
    let vpn_config = &args[3];

    // 2. Lógica condicional principal
    if version_openvpn == "2" {
        // Código para openvpn 2: sudo openvpn $VPN_CONFIG &; sudo -k

        // Inicia openvpn com sudo em background (spawn)
        eprintln!("Iniciando openvpn 2 em background com {}", vpn_config);
        match Command::new("openvpn")
            .arg("--config")
            .arg(vpn_config)
            .spawn()
        {
            Ok(_) => println!("OpenVPN 2 iniciado. Lembre-se que o processo roda em background."),
            Err(e) => {
                eprintln!("Erro ao iniciar OpenVPN 2 com sudo: {}", e);
                exit(1);
            }
        }
    } else if version_openvpn == "3" {
        // Código para openvpn 3
        if command_arg == "start" {
            // openvpn3 session-start --config "$VPN_CONFIG"
            println!("Iniciando conexão VPN...");
            let mut cmd = Command::new("openvpn3");
            cmd.arg("session-start")
                .arg("--config")
                .arg(vpn_config);
            match execute_command(cmd)
            {
                Ok(_) => println!("OpenVPN 3 iniciado com sucesso."),
                Err(e) => {
                    eprintln!("Erro ao iniciar OpenVPN 3: {}", e);
                    exit(1);
                }
            }
        } else if command_arg == "stop" {
            // Lógica para parar a sessão: openvpn3 sessions-list e parse

            let output = match Command::new("openvpn3")
                .arg("sessions-list")
                .output()
            {
                Ok(out) => out,
                Err(e) => {
                    eprintln!("Erro ao executar 'openvpn3 sessions-list': {}", e);
                    exit(1);
                }
            };

            let output_str = match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Erro ao decodificar a saída do 'openvpn3 sessions-list': {}", e);
                    exit(1);
                }
            };

            // Lógica de parsing (substituindo o awk)
            let mut session_paths: Vec<String> = Vec::new();
            let mut current_path: Option<String> = None;

            let reader = BufReader::new(output_str.as_bytes());

            for line_result in reader.lines() {
                let line = match line_result {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("Erro ao ler linha: {}", e);
                        continue;
                    }
                };

                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                if parts.len() >= 2 && parts[0] == "Path:" {
                    // Salva o Path
                    current_path = Some(parts[1].to_string());
                } else if parts.len() >= 3 && parts[0] == "Config" && parts[1] == "name:" {
                    // O nome da configuração é o terceiro campo ($3 no awk)
                    if parts[2] == vpn_config {
                        // Se a configuração corresponder, adiciona o Path salvo
                        if let Some(path) = current_path.take() { // take move e limpa
                            session_paths.push(path);
                        }
                    }
                    // Resetamos current_path para a próxima sessão
                    current_path = None;
                }
            }

            if session_paths.is_empty() {
                println!("Nenhuma sessão encontrada para {}", vpn_config);
                exit(1);
            }

            // Desconecta as sessões encontradas
            for session in &session_paths {
                let mut cmd = Command::new("openvpn3");
                cmd.arg("session-manage")
                    .arg("--session-path")
                    .arg(session)
                    .arg("--disconnect");
                match execute_command(cmd)
                {
                    Ok(_) => println!("Sessão {} desconectada.", session),
                    Err(e) => eprintln!("Erro ao desconectar sessão {}: {}", session, e),
                }
            }
        } else {
            print_usage_and_exit(program_name);
        }
    } else {
        // Versão inválida
        print_usage_and_exit(program_name);
    }
}