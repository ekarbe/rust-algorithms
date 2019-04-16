pub fn sort(arr: &mut [i32]) -> &mut [i32] {
  let mut i = 1;
  while i < arr.len() {
    let x = arr[i];
    let mut j = i - 1;
    while j > 0 && arr[j] > x {
      arr[j+1] = arr[j];
      if j == 0 {
        break;
      }
      j -= 1;
    }
    arr[j+1] = x;
    i += 1;
  }
 return arr;
}

#[test]
fn average() {
  assert_eq!(sort(&mut [2,4,1,5]), &mut [1,2,4,5]);
  assert_eq!(sort(&mut [23,14,52,10,2]), &mut [2,10,14,23,52]);
  assert_eq!(sort(&mut [324,1,34,23,37]),&mut [1,23,34,37,324]);
  assert_eq!(sort(&mut [436,23,25,124,235,234]),&mut [23,25,124,234,235,436]);
}
#[test]
fn best() {
  assert_eq!(sort(&mut [1,2,3,4]), &mut [1,2,3,4]);
}
#[test]
fn worst() {
  assert_eq!(sort(&mut [4,3,2,1]), &mut [1,2,3,4]);
}