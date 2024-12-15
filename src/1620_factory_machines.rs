use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (_, goal) = line.split_once(' ').unwrap();
    let goal = goal.parse::<u64>()?;

    let machines = lines
        .next()
        .unwrap()?
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let (mut low, mut high) = (0, machines.iter().min().unwrap() * goal);
    let mut time = 0;
    let mut mid;
    let mut produced: u64;

    while low <= high {
        mid = (low + high) / 2;
        produced = machines.iter().map(|x| mid / x).sum();
        if produced >= goal {
            time = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    writeln!(output, "{time}")?;

    Ok(())
}
