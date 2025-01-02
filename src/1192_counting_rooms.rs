use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (n, m) = line.split_once(' ').unwrap();
    let n = n.parse::<usize>()?;
    let m = m.parse::<usize>()?;

    let mut building = Vec::with_capacity(n * m);

    for line in lines {
        let line = line?;
        building.extend(line.chars());
    }

    let mut components = 0;

    for row in 0..n {
        for col in 0..m {
            // println!("ij = {}, i = {}, j = {}", row * m + col, row, col);
            if building[row * m + col] == '.' {
                dfs(row, col, &mut building, m, n);
                components += 1;
            }
        }
    }

    writeln!(output, "{components}")?;

    Ok(())
}

fn dfs(row: usize, col: usize, building: &mut [char], m: usize, n: usize) {
    let mut to_visit = vec![(row, col)];

    while let Some((row, col)) = to_visit.pop() {
        if building[row * m + col] == '.' {
            building[row * m + col] = 'x';

            if row > 0 {
                to_visit.push((row - 1, col))
            }
            if col > 0 {
                to_visit.push((row, col - 1))
            }
            if row + 1 < n {
                to_visit.push((row + 1, col))
            }
            if col + 1 < m {
                to_visit.push((row, col + 1))
            }
        }
    }
}
