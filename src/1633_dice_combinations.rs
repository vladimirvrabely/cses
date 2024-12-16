use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap()?.parse::<usize>()?;
    let mut counts = Vec::with_capacity(n);
    counts.push(1);

    let mut x: usize;
    for _ in 0..n {
        x = counts.iter().rev().take(6).sum::<usize>() % 1_000_000_007;
        counts.push(x);
    }

    writeln!(output, "{}", counts.last().unwrap())?;

    Ok(())
}
