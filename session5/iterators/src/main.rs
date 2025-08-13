struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Counter {
    fn len(&self) -> usize {
        self.max as usize
    }
}

fn main() {
    let numbers: Vec<u32> = Counter::new(10).collect();
    println!("{numbers:?}");
}