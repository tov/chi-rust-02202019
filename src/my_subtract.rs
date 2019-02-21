//! Testing my buggy subtraction function.

pub fn my_subtract(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        b -= 1;
        a -= 1;
    }

    a
}

