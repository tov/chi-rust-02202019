//! Testing my buggy quicksort.

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot = slice.len() - 1;
    let mut i = 0;

    for j in 0 .. pivot {
        if slice[j] < slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot);

    i
}

pub fn my_sort<T: Ord>(slice: &mut [T])
{
    if slice.len() > 2 {
        let pivot = partition(slice);
        my_sort(&mut slice[.. pivot]);
        my_sort(&mut slice[pivot + 1 ..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    fn is_sorted<T: Ord>(v: &[T]) -> bool {
        v.iter().zip(v.iter().skip(1))
            .all(|(curr, next)| curr <= next)
    }

    quickcheck!{

        fn prop_sort_i32(original: Vec<i32>) -> bool {
            let mut sorted = original;
            my_sort(&mut sorted);
            is_sorted(&sorted)
        }

        fn prop_sort_string(original: Vec<String>) -> bool {
            let mut sorted = original;
            my_sort(&mut sorted);
            is_sorted(&sorted)
        }

    }
}
