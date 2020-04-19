use rust_by_example::{mobile::*, web::*};

struct Point {
    x: i32,
    y: f32,
}

impl Computers for Point {
    fn int(&self) -> i32 {
        self.x
    }
    fn float(&self) -> f32 {
        self.y
    }
}
fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    hello();

    let point = Point { x: 10, y: 5.2 };
    println!(
        "point have value x = {}, y = {}",
        point.int(),
        point.float()
    );
}
