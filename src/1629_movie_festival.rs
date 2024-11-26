use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap()?.parse::<usize>()?;

    let mut movies = Vec::with_capacity(n);
    for line in lines {
        let line = line?;
        let (start, end) = line.split_once(' ').unwrap();
        let start = start.parse::<i64>()?;
        let end = end.parse::<i64>()?;
        movies.push((start, end))
    }
    movies.sort_unstable_by_key(|x| x.1);

    let mut total = 0;
    let mut end = 0;
    for m in movies {
        if end <= m.0 {
            total += 1;
            end = m.1
        }
    }

    writeln!(output, "{total}")?;

    Ok(())
}
