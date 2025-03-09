#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

pub fn run() {
    let p1 = Person { name: "Alice".to_string(), age: 30 };
    let p2 = p1.clone();
    
    println!("{:?}", p1);
    println!("Equal: {}", p1 == p2);
}
