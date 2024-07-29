use sorting::bubble_sort::bubble_sort;

fn main() {
    let mut input = [5, 4, 3, 2, 1];
    println!("Input: {:?}", input);
    bubble_sort(&mut input);
    println!("Output: {:?}", input);
}
