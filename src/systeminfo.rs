use std::process::Command;

pub fn query_bios(hostname: &String) -> Option<String> {
    let query = Command::new("systeminfo")
        .arg("/s")
        .arg(hostname)
        .output();

    match query {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let splices = stdout.lines();

            //println!("comando executado ok!");
            //println!("{}", stdout);

            for line in splices {
                if line.contains("BIOS:") {
                    let bios_splices = line.split_whitespace();
                    let mut result_str: String = String::new();

                    for value in bios_splices {
                        if value.contains(".") {
                            result_str += value;
                        }

                        if value.contains("/") {
                            result_str += value;
                        }
                    }

                    return Some(result_str);
                }
            }

            return None;
        }

        Err(_) => None
    }
}

/*
println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
*/