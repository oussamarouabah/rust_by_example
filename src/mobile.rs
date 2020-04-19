pub trait Computers {
    fn int(&self) -> i32;
    fn float(&self) -> f32;
    fn default() {
        println!("hi this is the default implementation of this function");
    }
}

pub fn hello() {
    println!("hello mobile");
}
