use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();

    io::stdin().read_line(&mut buff)?;
    let n: usize = buff.trim().parse()?;
    buff.clear();

    io::stdin().read_line(&mut buff)?;
    let mut v = buff
        .split_whitespace()
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut previous = v[0];
    let mut moves = 0;
    let mut inc;

    for i in 1..n {
        inc = (previous - v[i]).max(0);
        moves += inc;
        v[i] += inc;
        previous = v[i];
    }
    println!("{}", moves);

    Ok(())
}
