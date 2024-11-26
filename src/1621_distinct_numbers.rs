use std::collections::HashSet;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut unique = HashSet::new();
    let mut buff = String::new();

    io::stdin().read_line(&mut buff)?;
    let _n: usize = buff.trim().parse()?;
    buff.clear();

    io::stdin().read_line(&mut buff)?;

    for x in buff.split_whitespace() {
        unique.insert(x.parse::<u64>()?);
    }
    println!("{}", unique.len());

    Ok(())
}
