use getrandom;

use std::io;

const DICEWARE: &str = include_str!("diceware8k.txt");

fn get_random() -> Result<u16, getrandom::Error> {
    let mut buf: [u8; 2] = [0; 2];
    getrandom::getrandom(&mut buf)?;
    Ok((buf[0] as u16) + ((buf[1] as u16 & 31) << 8))
}

fn dw_dictionary() -> Vec<&'static str> {
    DICEWARE.split("\n").collect()
}

fn parse_num_words(num: &std::ffi::OsStr) -> io::Result<u64> {
    let s = num.to_str().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "unparseable command line argument.",
    ))?;
    s.parse::<u64>().or_else(|e| {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Couldn't parse number: {:?}", e),
        ))
    })
}

fn main() -> io::Result<()> {
    let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "1 argument required"));
    }
    assert!(args.len() >= 2);

    let words = dw_dictionary();
    let word_count = parse_num_words(&args[1])?;
    let mut pass_words: Vec<&str> = vec![];
    for _ in 0..word_count {
        let idx = get_random()?;
        pass_words.push(words[idx as usize]);
    }
    println!("{}", pass_words.join(","));
    Ok(())
}
