/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let pivot_index = array.len() / 2;
    let pivot = array[pivot_index];

    let mut left: usize = 0;
    let mut right = array.len() - 1;

    while left < right {
        while array[left] < pivot {
            left += 1;
        }
        while array[right] > pivot {
            right -= 1;
        }

        if left < right {
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    sort(&mut array[..left]);
    sort(&mut array[left..]);
    //TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
