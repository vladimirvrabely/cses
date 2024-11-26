use std::io::{self, BufWriter, Write};

const OFFSET: usize = 'A' as usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let line = input.lines().next().unwrap()?;

    let mut output = BufWriter::new(io::stdout().lock());

    let mut counter = vec![0usize; 26];
    for c in line.chars() {
        counter[(c as usize) - OFFSET] += 1;
    }

    let mut no_solution = false;
    let mut even = Vec::with_capacity(line.len());
    let mut odd = Vec::with_capacity(line.len());

    for (i, cnt) in counter.into_iter().enumerate() {
        if cnt % 2 == 0 {
            for _ in 0..(cnt / 2) {
                even.push((i + OFFSET) as u8 as char);
            }
        } else if no_solution {
            writeln!(output, "NO SOLUTION")?;
            return Ok(());
        } else {
            for _ in 0..cnt {
                odd.push((i + OFFSET) as u8 as char)
            }
            no_solution = true;
        }
    }

    let s = even
        .iter()
        .chain(odd.iter())
        .chain(even.iter().rev())
        .collect::<String>();
    writeln!(output, "{s}")?;
    Ok(())
}
