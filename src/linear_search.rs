 // Simple linear search. iterate through all elements in an
 // array and compare each of them to the target.
 // If the target is found return the index.
 // If the target is not in the array return -1.
 pub fn index_of(arr: &mut [i32], tar: &mut i32) -> i32 {
    for (i, x) in arr.iter().enumerate() {
       if x == tar {
          return i as i32;
       }
    }
    return -1 as i32;
}

#[test]
fn in_array() {
        assert_eq!(index_of(&mut [1,2,3,4], &mut 4), 3);
}
#[test]
fn not_in_array() {
        assert_eq!(index_of(&mut [1,2,3,4], &mut 7), -1);
}
#[test]
fn worst() {
        assert_eq!(index_of(&mut [23,15,22,53,23,12], &mut 12), 5)
}
#[test]
fn best() {
        assert_eq!(index_of(&mut [23,15,22,53,23,12], &mut 23), 0)
}