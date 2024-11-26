use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let mut numbers = line.split(' ');
    let n = numbers.next().unwrap().parse::<usize>()?;
    let x = numbers.next().unwrap().parse::<i64>()?;

    let mut weights = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    weights.sort_unstable();

    let mut count = n;
    let mut i = 0;
    let mut j = n - 1;

    while i < j {
        if weights[i] + weights[j] <= x {
            i += 1;
            j -= 1;
            count -= 1;
        } else {
            j -= 1;
        }
    }
    writeln!(output, "{count}")?;

    Ok(())
}
