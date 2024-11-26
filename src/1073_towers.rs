use std::io::{self, BufWriter, Write};

fn binary_search(s: &[u64], x: u64) -> usize {
    let mut l = 0;
    let mut r = s.len();
    let mut m;

    while l < r {
        m = (l + r) / 2;
        if x < s[m] {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap()?.parse::<usize>()?;

    let mut towers = Vec::with_capacity(n);
    let mut i;

    let blocks = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    for block in blocks {
        i = binary_search(&towers, block);
        if i == towers.len() {
            towers.push(block)
        } else {
            towers[i] = block;
        }
    }
    writeln!(output, "{}", towers.len())?;

    Ok(())
}
