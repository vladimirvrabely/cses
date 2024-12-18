use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (_, money) = line.split_once(' ').unwrap();
    let money = money.parse::<usize>()?;

    let coins = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut values = vec![money + 1; money + 1];
    values[0] = 0;

    for m in 1..=money {
        for c in coins.iter() {
            if m >= *c && values[m - c] + 1 < values[m] {
                values[m] = values[m - c] + 1
            }
        }
    }

    let sum = *values.last().unwrap();
    if sum == money + 1 {
        writeln!(output, "-1")?;
    } else {
        writeln!(output, "{sum}")?;
    }
    Ok(())
}
