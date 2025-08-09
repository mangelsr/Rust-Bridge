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

fn use_wordlist(wordlist_file: File, target_hash: String) {
    // todo!(); // Usefull macro to set todos in code
    let mut file_buffer = BufReader::new(wordlist_file);
    let hash_bytes = hex::decode(target_hash).unwrap();

    // Memory re-use version
    let mut line = String::new();

    loop {
        let Ok(bytes_read) = file_buffer.read_line(&mut line) else {
            line.clear();
            continue;
        };

        if bytes_read == 0 {
            println!("Password not found");
            return;
        }

        let mut hasher = Md5::new();
        hasher.update(&line);
        let result = hasher.finalize();

        if hash_bytes == result.as_slice() {
            println!("Password found; is {line}");
            return;
        }

        line.clear();
    }
}
