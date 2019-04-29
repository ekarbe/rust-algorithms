// Searches a slice for the biggest value.
// Returns the result.
pub fn max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for n in arr {
        if *n > max {
            max = *n;
        }
    }
    return max;
}

// Searches a slie for the smalles value.
// Returns the result.
pub fn min(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for n in arr {
        if *n < min {
            min = *n;
        }
    }
    return min;
}

#[test]
fn best() {
    assert_eq!(max(&[100, 4, 3, 6, 2]), 100);
    assert_eq!(min(&[1, 32, 99, 23, 4]), 1);
}
#[test]
fn worst() {
    assert_eq!(max(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(min(&[5, 4, 3, 2, 1]), 1);
}
