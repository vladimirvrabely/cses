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

    let mut values = vec![0; x + 1];

    for i in 0..n {
        for j in (prices[i]..=x).rev() {
            values[j] = values[j].max(pages[i] + values[j - prices[i]])
        }
    }

    writeln!(output, "{}", values[x])?;

    Ok(())
}
