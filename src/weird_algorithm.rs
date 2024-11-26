use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;
    let mut n: i64 = buff.trim().parse()?;
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n += 2*n + 1;
        }
    }
    println!("1");
    Ok(())
}
