use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let _ = lines.next().unwrap()?;
    let line = lines.next().unwrap()?;

    let mut val;
    let mut best = std::i64::MIN;
    let mut curr = 0;

    for x in line.split(' ') {
        val = x.parse::<i64>()?;
        curr = (curr + val).max(val);
        best = best.max(curr);
    }

    writeln!(output, "{best}")?;

    Ok(())
}
