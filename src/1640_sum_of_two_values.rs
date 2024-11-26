use std::collections::BTreeMap;
use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (_n, s) = line.split_once(' ').unwrap();
    let _n = _n.parse::<usize>()?;
    let s = s.parse::<i64>()?;

    let v = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut index: BTreeMap<i64, i64> = BTreeMap::new();

    let mut complement;
    let mut pair = None;
    if s > 1 {
        for (i, x) in v.iter().enumerate() {
            complement = s - x;
            let j = index.get(&complement).clone();
            match j {
                Some(j) => {
                    pair = Some((i + 1, j + 1));
                    break;
                }
                None => {
                    index.insert(*x, i as i64);
                }
            }
        }
    }

    match pair {
        None => writeln!(output, "IMPOSSIBLE")?,
        Some(pair) => writeln!(output, "{} {}", pair.0, pair.1)?,
    }

    Ok(())
}
