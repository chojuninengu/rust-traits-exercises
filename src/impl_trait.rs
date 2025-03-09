fn count_up_to(n: u32) -> impl Iterator<Item = u32> {
    1..=n
}

pub fn run() {
    for num in count_up_to(5) {
        println!("{}", num);
    }
}
