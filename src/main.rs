use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::thread;

fn main() {
    let arquivo = Path::new("/sdcard/Android/config_cache.txt");

    if !arquivo.exists() {
        eprintln!("config: não encontrado");
        return;
    }
    println!("config: OK");

    let file = match File::open(arquivo) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERRO ao abrir: {e}");
            return;
        }
    };
    let render = BufReader::new(file);

    let caminhos: Vec<String> = render
        .lines()
        .map(|l| l.expect("linha inválida"))
        .collect();

    let mut threads = vec![];

    for par in caminhos.chunks(2) {
        let a = par[0].clone();
        let b = par.get(1).cloned();

        threads.push(thread::spawn(move || {
            clear(&a);
            if let Some(b) = b {
                clear(&b);
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("concluído");
}

fn clear(caminho: &str) {
    match fs::remove_dir_all(caminho) {
        Ok(_) => println!("OK: {caminho}"),
        Err(e) => println!("ERRO {caminho}: {e}"),
    }
}
