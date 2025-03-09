struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource is being dropped!");
    }
}

pub fn run() {
    let _res = Resource;
    println!("End of main");
}
