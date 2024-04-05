pub mod args;
pub mod systeminfo;

use std::io::Write;
use std::sync::mpsc;
use std::fs::File;
use systeminfo::query_bios;

struct QueryResult {
    host: String,
    result: Option<String>    
}

fn main() -> std::io::Result<()> {
    let hostnames = args::parse_args();
    let (tx, rx) = mpsc::channel::<QueryResult>();

    for host in hostnames {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            //crate::subnet::query_subnet(subnet, tx_clone);
            if let Some(bios_version) = query_bios(&host) {
                //let result_str = host + "," + &bios_version;
                let _ = tx_clone.send(QueryResult{host, result: Some(bios_version)});
            } else {
                let _ = tx_clone.send(QueryResult{host, result: None});
            }
        });
    }

    drop(tx);
    
    let mut file: File;
    //let file = File::create("saida.csv");
    if let Ok(file_ok) = File::create("saida.csv") {
        file = file_ok;
    } else {
        panic!("Unable to open output file!");
    }


    println!("\"Nome do host\",\"Versão da BIOS\",\"Data da BIOS\"\n");
    write!(file, "\"Nome do host\",\"Versão da BIOS\",\"Data da BIOS\"\n")?;
    
    for receiver in rx {
        if let Some(result) = receiver.result {
            println!("{},{}", receiver.host, result);
            write!(file, "{},{}\n", receiver.host, result)?;
        } else {
            println!("{},,", receiver.host);
            write!(file, "{},N/D,N/D\n", receiver.host)?;
        }
    }

    Ok(())
}