use md5::{Digest, Md5};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let word_list_path = std::env::args().nth(1).unwrap();
    let target_hash = std::env::args().nth(2).unwrap();

    let wordlist = File::open(word_list_path);
    match wordlist {
        Ok(file) => {
            use_wordlist(file, target_hash);
        }
        Err(e) => {
            eprintln!("{e}"); // eprint => send to stderr
            return;
        }
    }
}

fn use_wordlist(file: File, hash: String) {
    // todo!(); // Usefull macro to set todos in code
    let file_buffer = BufReader::new(file);
    let hash_bytes = hex::decode(hash).unwrap();

    for line in file_buffer.lines() {
        let Ok(line) = line else {
            continue;
        };

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut hasher = Md5::new();
        hasher.update(&line);
        let result = hasher.finalize();

        if hash_bytes == result.as_slice() {
            println!("Password found; is {line}");
            return;
        }
    }

    println!("Password not found");
}
