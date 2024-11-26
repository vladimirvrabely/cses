use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    let mut output = BufWriter::new(io::stdout().lock());

    lines.next().unwrap()?;

    for line in lines {
        let line = line?;

        let (a, b) = line.split_once(' ').unwrap();
        let a = a.parse::<usize>()?;
        let b = b.parse::<usize>()?;

        if (a + b) % 3 != 0 {
            writeln!(output, "NO")?;
        } else if a > 2 * b || b > 2 * a {
            writeln!(output, "NO")?
        } else {
            writeln!(output, "YES")?
        }
    }

    Ok(())
}
