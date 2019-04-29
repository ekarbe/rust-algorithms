pub fn sort(arr: &mut [i32]) -> &mut [i32] {
    let begin_i = 0;
    let end_i = arr.len() - 1;
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in begin_i..end_i {
            if arr[i] > arr[i + 1] {
                arr.swap(i + 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    return arr;
}

#[test]
fn average() {
    assert_eq!(sort(&mut [2, 4, 1, 5]), &mut [1, 2, 4, 5]);
    assert_eq!(sort(&mut [23, 14, 52, 10, 2]), &mut [2, 10, 14, 23, 52]);
    assert_eq!(sort(&mut [324, 1, 34, 23, 37]), &mut [1, 23, 34, 37, 324]);
    assert_eq!(
        sort(&mut [436, 23, 25, 124, 235, 234]),
        &mut [23, 25, 124, 234, 235, 436]
    );
}
#[test]
fn best() {
    assert_eq!(sort(&mut [1, 2, 3, 4, 5, 6]), &mut [1, 2, 3, 4, 5, 6]);
}
#[test]
fn worst() {
    assert_eq!(sort(&mut [6, 5, 4, 3, 2, 1]), &mut [1, 2, 3, 4, 5, 6]);
}
