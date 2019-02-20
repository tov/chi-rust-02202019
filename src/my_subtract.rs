//! Testing my buggy subtraction function.

pub fn my_subtract(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        b -= 1;
        a -= 1;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{quickcheck, TestResult};

    quickcheck!{

        fn prop_add_subtract_non_negative(a: i32, b: i32) -> TestResult {
            if b < 0 {
                return TestResult::discard();
            }

            TestResult::from_bool( my_subtract(a, b) + b == a )
        }

    }
}
