use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./wordlist-german.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut len: usize;
    for word in contents.split('\n') {
        len = word.len();
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("wordlist-{}.txt", &len))?;
        file.write_all(format!("{}\n", word).as_bytes())?;
    }
    Ok(())
}
