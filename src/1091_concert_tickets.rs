use std::collections::BTreeSet;
use std::io::{self, BufWriter, Write};
use std::ops::Bound::Included;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let mut numbers = line.split(' ');
    let n = numbers.next().unwrap().parse::<usize>()?;
    let _m = numbers.next().unwrap().parse::<usize>()?;

    let mut prices = lines
        .next()
        .unwrap()?
        .split(' ')
        .enumerate()
        .map(|(i, x)| match x.parse::<i64>() {
            Ok(x) => Ok((x, i)),
            Err(e) => Err(e),
        })
        .collect::<Result<BTreeSet<_>, _>>()?;

    let customers = lines
        .next()
        .unwrap()?
        .split(' ')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    for c in customers.into_iter() {
        if prices.is_empty() {
            writeln!(output, "-1")?;
        } else {
            let purchase = prices
                .range((Included((0, 0)), Included((c, n))))
                .rev()
                .next()
                .cloned();
            match purchase {
                Some(purchase) => {
                    prices.remove(&purchase);
                    writeln!(output, "{}", purchase.0)?;
                }
                None => writeln!(output, "-1")?,
            };
        }
    }

    Ok(())
}
