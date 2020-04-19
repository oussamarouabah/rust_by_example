fn main() {
    // this is a closure.
    let one = || 1;
    let nbr = cry(one);
    println!("{}", nbr);
}

fn cry<T>(f: T) -> i32
where
    T: Fn() -> i32,
{
    f()
}
