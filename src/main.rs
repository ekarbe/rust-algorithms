mod linear_search;
mod insertion_sort;

fn main() {
    println!("Run 'cargo test' to test all algorithms!");
    insertion_sort::sort(&mut [23,14,52,10,2]);
}
