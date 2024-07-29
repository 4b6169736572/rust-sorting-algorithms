use sorting::selection_sort::selection_sort;

fn main() {
    let mut input = [5, 4, 3, 2, 1];
    println!("Input: {:?}", input);
    selection_sort(&mut input);
    println!("Output: {:?}", input);
}
