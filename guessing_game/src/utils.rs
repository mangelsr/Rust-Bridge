use std::io::{Write, stdin, stdout};

pub fn input(text: &str) -> String {
    print!("{text}");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim_end_matches(['\n', '\r']).to_string();
    buffer
}
