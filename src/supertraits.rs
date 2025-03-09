use std::fmt::Debug;

trait Printable: Debug {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("{:?}", self);
    }
}

pub fn run() {
    let num: i32 = 42;
    num.print();
}
