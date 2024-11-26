use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap()?.parse::<usize>()?;

    let mut arr = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let (_, median, _) = arr.select_nth_unstable(n / 2);
    let median = median.clone();

    let sum: i64 = arr.iter().map(|x| (*x - median).abs()).sum();

    writeln!(output, "{sum}")?;

    Ok(())
}
