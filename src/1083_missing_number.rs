use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();

    io::stdin().read_line(&mut buff)?;
    let n: usize = buff.trim().parse()?;
    buff.clear();

    io::stdin().read_line(&mut buff)?;
    let mut v = vec![false; n];

    for x in buff.split_whitespace() {
        v[x.parse::<usize>()? - 1] = true;
    }
    let index = v
        .into_iter()
        .enumerate()
        .find(|&(_, b)| !b)
        .map(|(i, _)| i)
        .unwrap();
    println!("{}", index + 1);

    Ok(())
}
