use std::{
    collections::VecDeque,
    io::{self, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let line = lines.next().unwrap()?;
    let (n, m) = line.split_once(' ').unwrap();
    let n = n.parse::<usize>()?;
    let m = m.parse::<usize>()?;

    let mut labyrinth = vec![];

    for line in lines {
        let line = line?;
        labyrinth.push(line.chars().collect::<Vec<char>>());
    }

    let mut a = (0, 0);
    let mut b = (0, 0);
    for row in 0..n {
        for col in 0..m {
            if labyrinth[row][col] == 'A' {
                a = (row as i64, col as i64);
            }
            if labyrinth[row][col] == 'B' {
                b = (row as i64, col as i64);
            }
        }
    }

    let mut visited = vec![vec![false; m]; n];
    let mut to_visit = VecDeque::new();
    let mut step = vec![vec![None; m]; n];
    let mut connected = false;

    const DIRECTION_ROW: [i64; 4] = [-1, 0, 1, 0];
    const DIRECTION_COL: [i64; 4] = [0, -1, 0, 1];
    const DIRECTION_CHAR: [char; 4] = ['U', 'L', 'D', 'R'];

    to_visit.push_back(a);
    let mut r;
    let mut c;

    while let Some((row, col)) = to_visit.pop_front() {
        if (row, col) == b {
            connected = true;
            break;
        }

        if visited[row as usize][col as usize] {
            continue;
        }

        visited[row as usize][col as usize] = true;
        for neigbour in 0..4 {
            r = row + DIRECTION_ROW[neigbour];
            c = col + DIRECTION_COL[neigbour];
            if 0 <= r
                && r < n as i64
                && 0 <= c
                && c < m as i64
                && !visited[r as usize][c as usize]
                && labyrinth[r as usize][c as usize] != '#'
            {
                step[r as usize][c as usize] = Some(neigbour);
                to_visit.push_back((r, c));
            }
        }
    }

    if connected {
        let mut path = vec![];
        let (mut row, mut col) = b;
        let mut s;
        while (row, col) != a {
            s = step[row as usize][col as usize].expect("Algorithm error");
            path.push(DIRECTION_CHAR[s]);
            row -= DIRECTION_ROW[s];
            col -= DIRECTION_COL[s];
        }
        let path: String = path.iter().rev().collect();

        writeln!(output, "YES")?;
        writeln!(output, "{}", path.len())?;
        writeln!(output, "{}", path)?;
    } else {
        writeln!(output, "NO")?;
    }

    Ok(())
}
