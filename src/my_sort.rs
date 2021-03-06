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

    fn sort_fun<T: Ord>(mut v: Vec<T>) -> Vec<T> {
        my_sort(&mut v);
        v
    }

    #[test]
    fn sort_single() {
        assert_eq!( sort_fun(vec![5]), &[5] );
    }

    #[test]
    fn sort_1234() {
        assert_eq!( sort_fun(vec![1, 2, 3, 4]), &[1, 2, 3, 4] );
    }

    #[test]
    fn sort_4321() {
        assert_eq!( sort_fun(vec![3, 4, 2, 1]), &[1, 2, 3, 4] );
    }

}
