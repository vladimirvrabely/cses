use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let mut numbers = line.split(' ');
    let _n = numbers.next().unwrap().parse::<usize>()?;
    let _m = numbers.next().unwrap().parse::<usize>()?;
    let k = numbers.next().unwrap().parse::<i64>()?;

    let mut applicants = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    applicants.sort();

    let mut apartments = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    apartments.sort();

    let mut i = 0;
    let mut j = 0;
    let mut applicant;
    let mut apartment;
    let mut count = 0;

    while i < applicants.len() && j < apartments.len() {
        applicant = applicants[i];
        apartment = apartments[j];

        if applicant > apartment + k {
            j += 1;
        } else if applicant < apartment - k {
            i += 1;
        } else {
            i += 1;
            j += 1;
            count += 1;
        }
    }
    writeln!(output, "{count}")?;

    Ok(())
}
