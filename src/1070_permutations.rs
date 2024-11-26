use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();

    io::stdin().read_line(&mut buff)?;
    let n: usize = buff.trim().parse()?;

    match n {
        1 => println!("1"),
        2 | 3 => println!("NO SOLUTION"),
        n => (2..=n)
            .step_by(2)
            .chain((1..=n).step_by(2))
            .for_each(|x| print!("{} ", x)),
    }
    Ok(())
}
