use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut output = BufWriter::new(io::stdout().lock());

    let n = input.lines().next().unwrap()?.parse::<u64>()?;
    let mut m = 5u64;
    let mut zeros = 0u64;
    while m <= n {
        zeros += n / m;
        m *= 5;
    }
    writeln!(output, "{zeros}")?;

    Ok(())
}
