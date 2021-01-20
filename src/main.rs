use getrandom;

const DICEWARE: &str = include_str!("diceware8k.txt");

fn get_random() -> Result<u16, getrandom::Error> {
    let mut buf: [u8; 2] = [0; 2];
    getrandom::getrandom(&mut buf)?;
    Ok((buf[0] as u16) + ((buf[1] as u16 & 31) << 8))
}

fn dw_dictionary() -> Vec<&'static str> {
    DICEWARE.split("\n").collect()
}

// first arg: number of words
fn main() -> std::io::Result<()> {
    let mut args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    assert!(args.len() >= 2);

    let words = dw_dictionary();
    let word_count: u64 = args.remove(1).into_string().unwrap().parse().unwrap();
    let mut pass_words: Vec<&str> = vec![];
    for _ in 0..word_count {
	let idx = get_random()?;
	pass_words.push(words[idx as usize]);
    }
    println!("{}", pass_words.join(","));
    Ok(())
}
