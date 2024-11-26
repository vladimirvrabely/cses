use std::{
    collections::HashMap,
    io::{self, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut lines = input.lines();
    // Flush all at once
    let mut output = BufWriter::new(io::stdout().lock());

    let _n = lines.next().unwrap()?.parse::<usize>()?;

    let mut length = 0usize;
    let mut start = 0usize;
    let mut last = HashMap::<u32, usize>::new();

    let playlist = lines.next().unwrap()?;
    let playlist = playlist.split(' ');
    for (end, song) in playlist.enumerate() {
        let song = song.parse::<u32>()?;
        if let Some(index) = last.get(&song) {
            if start <= *index {
                start = index + 1;
            }
        }
        last.insert(song, end);
        length = length.max(end - start + 1);
    }
    writeln!(output, "{length}")?;

    Ok(())
}
