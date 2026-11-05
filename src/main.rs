use std::io::{BufRead, BufReader};
use std::fs;
use std::path::Path;

fn main() {
    if let Err(e) = clear() {
        eprintln!("ERRO {e}");
    }
}


//deleta pastas
fn clear_cache(pasta: &str) {
    let p = Path::new(pasta);

    if !p.exists() {
        return;
    }

    match fs::remove_dir_all(p) {
        Ok(_) => println!("OK"),
        Err(e) => println!("ERRO em {pasta} e {e}"),  
    }
}

//nova versao
fn clear() -> std::io::Result<()> {

    let arquivo = Path::new("/sdcard/Android/config_cache.txt");
    
    if arquivo.exists() {
        println!("true");
        let file =  std::fs::File::open(arquivo)?;

        let ler = BufReader::new(file);

        for linha in ler.lines() {
            let linha = linha?;
            match fs::remove_dir_all(linha) {
                Ok(_) => println!("OK"),
                Err(e) => println!("ERRO {e}"),
            }
        }
        Ok(()) 
    }
    else {
        fs::write(arquivo, "")?;
        println!("ARQUIVO CRIADO");
        println!("coloque os caminhos de pastas no");
        println!("/sdcard/Android/config_cache.txt");
        Ok(())
    }
}
