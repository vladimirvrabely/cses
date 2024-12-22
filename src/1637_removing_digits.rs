use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut output = BufWriter::new(io::stdout().lock());

    let n = input.lines().next().unwrap()?.parse::<usize>()?;
    let mut dp = vec![n; n + 1];
    let mut x: usize;
    let mut d: usize;
    dp[0] = 0;

    for i in 1..=n {
        x = i;
        while x > 0 {
            d = x % 10;
            dp[i] = dp[i].min(dp[i - d] + 1);
            x /= 10;
        }
    }

    writeln!(output, "{}", dp[n])?;

    Ok(())
}
