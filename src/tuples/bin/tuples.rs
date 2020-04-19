use std::fmt::{self, Display, Formatter};

// The following struct is for the activity.
#[derive(Debug)]
pub struct Matrix(pub f32, pub f32, pub f32, pub f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} ) \n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// Tuples can be used as function arguments and as return values
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

pub fn transpose(x: Matrix) -> Matrix {
    Matrix(x.0, x.2, x.1, x.3)
}
