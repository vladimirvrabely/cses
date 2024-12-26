use std::io::{self, BufWriter, Write};

#[derive(Clone, Debug)]
struct Task {
    duration: i64,
    deadline: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let n = lines.next().unwrap().unwrap().parse::<usize>()?;

    let mut tasks = Vec::with_capacity(n);

    for line in lines {
        let line = line?;
        let (duration, deadline) = line.split_once(' ').unwrap();
        let duration = duration.parse::<i64>()?;
        let deadline = deadline.parse::<i64>()?;
        tasks.push(Task { duration, deadline });
    }
    tasks.sort_unstable_by_key(|t| t.duration);

    let reward = tasks
        .iter()
        .fold((0, 0), |(mut finished, mut reward), task| {
            finished += task.duration;
            reward += task.deadline - finished;
            (finished, reward)
        })
        .1;

    writeln!(output, "{}", reward)?;

    Ok(())
}
