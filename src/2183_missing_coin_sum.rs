use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let _ = lines.next().unwrap()?.parse::<usize>()?;

    let mut coins = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    coins.sort_unstable();
    let mut x = 1;

    for c in coins {
        if c > x {
            break;
        } else {
            x += c;
        }
    }

    writeln!(output, "{x}")?;

    Ok(())
}
