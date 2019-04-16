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
fn normal() {
  assert_eq!(sort(&mut [23,14,52,10,2]), &mut [2,10,14,23,52])
}
#[test]
fn best() {
  assert_eq!(sort(&mut [1,2,3,4]), &mut [1,2,3,4]);
}
#[test]
fn worst() {
  assert_eq!(sort(&mut [4,3,2,1]), &mut [1,2,3,4]);
}