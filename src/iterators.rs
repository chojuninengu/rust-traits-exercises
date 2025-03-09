struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let mut counter = Counter { count: 0 };
    for val in counter {
        println!("{}", val);
    }
}
 