use sorting::insertion_sort::insertion_sort;

fn main() {
    let mut input = [5, 4, 3, 2, 1];
    println!("Input: {:?}", input);
    insertion_sort(&mut input);
    println!("Output: {:?}", input);
}
