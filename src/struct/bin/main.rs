#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left: top,
            bottom_right: bottom,
        } = self;
        (top.y - bottom.y) * (bottom.x - top.x)
    }
}

fn square(point: Point, nbr: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y + nbr,
        },
        bottom_right: Point {
            x: point.x + nbr,
            y: point.y,
        },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    let bottom_right: Point = Point { x: 5.2, ..point };
    println!("secong point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area of rectangle is  {:?}", _rectangle.rect_area());

    let rect = square(Point { x: 10.2, y: 5.0 }, 10.2);

    println!("area of the new rectangle is  {:?}", rect.rect_area());
}
