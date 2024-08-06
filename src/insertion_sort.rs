pub fn insertion_sort<T: PartialOrd + Clone>(input: &mut [T]) {
    let len = input.len();

    if len == 1 {
        return;
    }

    for start in 1..len {
        let value = input[start].clone();

        // Push item to correct place in list by swapping forward items greater than `value`
        for offset in 1..=start {
            if input[start - offset] > value {
                input.swap(start - offset, start - offset + 1)
            }
        }
    }
}
