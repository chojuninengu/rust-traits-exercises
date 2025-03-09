trait Speak {
    fn speak(&self);
}

struct Person;
struct Dog;

impl Speak for Person {
    fn speak(&self) {
        println!("Hello!");
    }
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

pub fn run() {
    let speakers: Vec<Box<dyn Speak>> = vec![Box::new(Person), Box::new(Dog)];
    for speaker in speakers {
        speaker.speak();
    }
}
