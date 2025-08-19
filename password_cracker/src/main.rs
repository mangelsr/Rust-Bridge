use md5::{Digest, Md5};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::atomic::{AtomicBool, Ordering},
};

const NUM_CORES: usize = 8;
static FOUND: AtomicBool = AtomicBool::new(false);

fn main() {
    let word_list_path = std::env::args().nth(1).unwrap();
    let target_hash = std::env::args().nth(2).unwrap();

    std::thread::scope(|s| {
        for i in 0..NUM_CORES {
            let word_list_path = word_list_path.clone();
            let target_hash = target_hash.clone();

            s.spawn(move || {
                let wordlist = File::open(&word_list_path).unwrap();
                use_wordlist(&wordlist, &target_hash, i);
            });
        }
    });
}

fn use_wordlist(wordlist_file: &File, target_hash: &str, thread_id: usize) {
    let mut file_buffer = BufReader::new(wordlist_file);
    let hash_bytes = hex::decode(target_hash).unwrap();
    let mut line = String::new();
    let mut line_number = 0;

    loop {
        if FOUND.load(Ordering::Relaxed) {
            return;
        }

        line.clear();
        let bytes_read = match file_buffer.read_line(&mut line) {
            Ok(n) => n,
            Err(_) => continue,
        };

        if bytes_read == 0 {
            if thread_id == 0 {
                println!("Password not found");
            }
            return;
        }

        if line_number % NUM_CORES == thread_id {
            let mut hasher = Md5::new();
            hasher.update(line.trim_end().as_bytes());
            let result = hasher.finalize();

            if hash_bytes == result.as_slice() {
                FOUND.store(true, Ordering::Relaxed);
                println!("Password found: {}", line.trim_end());
                return;
            }
        }

        line_number += 1;
    }
}
