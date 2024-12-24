use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n: usize = lines.next().unwrap()?.parse()?;

    let mut board = Vec::with_capacity(n * n);

    for line in lines {
        let line = line?;
        for c in line.chars() {
            board.push(c);
        }
    }

    let mut dp = vec![0; n * n];

    if board[0] == '*' {
        writeln!(output, "0")?;
    } else {
        dp[0] = 1;
        for row in 0..n {
            for col in 0..n {
                if board[row + n * col] == '.' {
                    if row >= 1 {
                        dp[row + n * col] += dp[(row - 1) + n * col];
                    }
                    if col >= 1 {
                        dp[row + n * col] += dp[row + n * (col - 1)];
                    }
                    dp[row + n * col] %= 1_000_000_007;
                }
            }
        }
        writeln!(output, "{}", dp[n * n - 1])?;
    }

    Ok(())
}
