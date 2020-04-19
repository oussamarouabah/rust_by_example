//enum Foo {Bar}

// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        // n if n > 13 && n < 19 => println!("I'm a teen of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 0..=42) => println!("The Answer: {}!", n),
        Some(n @ 43..=60) => println!("The Jok is : {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }

    // let a = Foo::Bar;

    // // Variable a matches Foo::Bar
    // if let Foo::Bar = a {
    // // ^-- this causes a compile-time error. Use `if let` instead.
    //     println!("a is foobar");
    // }

    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
            continue;
        }
        println!("`i` is `{:?}`. Try again...", i);
        optional = Some(i + 1);
    }
}
