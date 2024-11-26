// TODO: use dynamic programming
use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n: usize = lines.next().unwrap()?.parse()?;

    let v = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let total: i64 = v.iter().sum();
    let mut diff = total;

    // Iterate over powerset
    for mask in 0..(1 << n) {
        let partial: i64 = v
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, x)| x)
            .sum();

        diff = diff.min((total - 2 * partial).abs());
    }
    writeln!(output, "{diff}")?;

    Ok(())
}
