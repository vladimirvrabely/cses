use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    lines.next();

    for line in lines {
        let line = line?;
        let (row, col) = line.split_once(' ').unwrap();
        let row = row.parse::<usize>()?;
        let col = col.parse::<usize>()?;

        let k = row.max(col);
        // Diagonal value
        let d = k * (k - 1) + 1;
        let value = if k % 2 == 0 {
            d - (col - row)
        } else {
            d + (col - row)
        };
        writeln!(output, "{value}")?;
    }
    Ok(())
}
