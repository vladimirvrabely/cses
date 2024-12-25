use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (n, x) = line.split_once(' ').unwrap();
    let n = n.parse::<usize>()?;
    let x = x.parse::<usize>()?;

    let prices = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let pages = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut values = vec![0; (n + 1) * (x + 1)];

    for i in 1..=n {
        for j in 0..=x {
            if prices[i - 1] > j {
                values[i * (x + 1) + j] = values[(i - 1) * (x + 1) + j];
            } else {
                values[i * (x + 1) + j] = values[(i - 1) * (x + 1) + j]
                    .max(values[(i - 1) * (x + 1) + j - prices[i - 1]] + pages[i - 1]);
            }
        }
    }

    writeln!(output, "{}", values[(n + 1) * (x + 1) - 1])?;

    Ok(())
}
