use getrandom;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_random() -> Result<u16, getrandom::Error> {
    let mut buf: [u8; 2] = [0; 2];
    getrandom::getrandom(&mut buf)?;
    Ok((buf[0] as u16) + ((buf[1] as u16 & 31) << 8))
}

fn read_diceware<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let result = BufReader::new(file).lines().collect::<std::io::Result<Vec<_>>>()?;
    assert_eq!(result.len(), 8192);
    Ok(result)
}

// should move this to a top level constant?
fn get_embedded_diceware() -> &'static str {
    include_str!("diceware8k.txt")
}

fn get_vec() -> Vec<&'static str> {
    get_embedded_diceware().split("\n").collect()
}

// first arg: path to diceware file
// second arg: number of words
fn main() -> std::io::Result<()> {
    // what do we want to get?
    // we want 13 bits. so i need to get 2 bytes. and shift by 3 to the right.
    let mut args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    assert!(args.len() >= 2);

    let words = get_vec();
    let word_count: u64 = args.remove(1).into_string().unwrap().parse().unwrap();
    let mut pass_words: Vec<&str> = vec![];
    for _ in 0..word_count {
	let idx = get_random()?;
	pass_words.push(words[idx as usize]);
    }
    println!("{}", pass_words.join(","));
    Ok(())
}
