use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff)?;

    let mut longest = 0;
    let mut c = buff.chars().next().unwrap();
    let mut n = 0;

    for x in buff.chars() {
        if x == c {
            n += 1;
        } else {
            longest = longest.max(n);
            n = 1;
            c = x;
        }
    }
    longest = longest.max(n);

    println!("{}", longest);

    Ok(())
}
