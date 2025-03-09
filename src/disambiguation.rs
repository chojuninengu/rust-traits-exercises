trait A {
    fn hello(&self);
}

trait B {
    fn hello(&self);
}

struct MyStruct;

impl A for MyStruct {
    fn hello(&self) {
        println!("Hello from A");
    }
}

impl B for MyStruct {
    fn hello(&self) {
        println!("Hello from B");
    }
}

pub fn run() {
    let obj = MyStruct;
    A::hello(&obj);
    B::hello(&obj);
}
