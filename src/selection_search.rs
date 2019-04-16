pub fn max(&mut [i32]) -> i32 {

}

pub fn min(&mut [i32]) -> i32 {

}

#[test]
fn best() {
  assert_eq!(max(&mut [100,4,3,6,2]), 100);
  assert_eq!(min(&mut [1,32,99,23,4]), 1);
}
#[test]
fn worst() {
  assert_eq!(max(&mut [1,2,3,4,5]), 5);
  assert_eq!(min(&mut [5,4,3,2,1]), 1);
}