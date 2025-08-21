use std::fs::read_to_string;
use std::{io::{BufRead, BufReader}, fs::File};
use memmap2::MmapOptions;

fn main() {
    // let now = std::time::Instant::now();    
    // let war_and_peace = read_to_string("../warandpeace.txt").unwrap();  // fit the entire file into memory; for bigger files is not so good
    // println!("Line count: {}", war_and_peace.lines().count());
    // println!("Completed in {} ms", now.elapsed().as_millis());

    // let now = std::time::Instant::now();    
    // let file = File::open("../warandpeace.txt").unwrap();
    // let buffered_reader = BufReader::new(file); // read a chunk at a time, makes a little slower but scale better if get bigger files
    // println!("Line count: {}", buffered_reader.lines().count());
    // println!("Completed in {} ms", now.elapsed().as_millis());

    let now = std::time::Instant::now();    
    let file = File::open("../warandpeace.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };  // unsafe since it is connecting to operating system command; it's the fastest but works better with big big files
    let buffered_reader = BufReader::new(&mmap[..]);
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}

// cargo add memmap2