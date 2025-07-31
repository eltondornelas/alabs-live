use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// syncronous context
/* fn main() {
    // // File hosts.txt must exist in the current path
    // if let Ok(lines) = read_lines("./hosts.txt") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for line in lines.map_while(Result::ok) {
    //         println!("{}", line);
    //     }
    // }

    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines("../warandpeace.txt") {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }

    println!("Read {} lines in {:.3} seconds", line_count, now.elapsed().as_secs_f32());
} */

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    // this function is not an asynchronous implementation
    let now = std::time::Instant::now();
    let mut line_count = 0;

    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }

    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    Ok(line_count)
}

async fn async_line_count(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines(); // the lines from tokio is not in fact an iterator, they are stream, steram are like an iterator except a stream is inherently asynchronous

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    println!(
        "Read {} lines in {:.3} seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );

    Ok(line_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Reading warandpeace...");
    let now = std::time::Instant::now();
    // let mut line_count = 0;
    let (c1, c2, ..) = tokio::join!(
        // line_count("../warandpeace.txt".to_string()),
        // line_count("../warandpeace.txt".to_string()) // this file input/output with line_count is not asynchronous, it is concurrent
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string()),
        async_line_count("../warandpeace.txt".to_string())
        // you can see that for a small quantity the sync code can be faster but when you start to grow, the assync code stay smooth while the sync starts to take a little longer
        // if you want to get a result as fast as possible (raw speed), spawn blocking or synchronous code may be your choice
        // if you goal is smoothly access a large dataset, assynchronous is your choice
    );
    println!("Total lines: {}", c1? + c2?);
    println!("in {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
