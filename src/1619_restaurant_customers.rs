use std::cmp::Ordering;
use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap().unwrap().parse::<usize>()?;

    let mut arrivals = Vec::with_capacity(n);
    let mut departures = Vec::with_capacity(n);

    for line in lines {
        let line = line?;
        let (a, d) = line.split_once(' ').unwrap();
        let a = a.parse::<i64>()?;
        let d = d.parse::<i64>()?;
        arrivals.push(a);
        departures.push(d);
    }

    arrivals.sort_unstable();
    departures.sort_unstable();

    let (mut state, mut max) = (0, 0);
    let (mut i, mut j) = (0, 0);

    while i < n && j < n {
        match arrivals[i].cmp(&departures[j]) {
            Ordering::Less => {
                i += 1;
                state += 1;
            }
            Ordering::Equal => {
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                j += 1;
                state -= 1;
            }
        }
        max = max.max(state);
    }

    writeln!(output, "{max}")?;

    Ok(())
}
