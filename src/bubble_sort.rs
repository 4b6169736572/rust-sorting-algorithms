pub fn bubble_sort<T: PartialOrd>(input: &mut [T]) {
    let len = input.len();

    if len == 1 {
        return;
    }

    loop {
        let mut swap = false;

        for index in 0..len - 1 {
            if input[index] <= input[index + 1] {
                continue;
            }
            input.swap(index, index + 1);
            swap = true;
        }
        if swap == false {
            break;
        }
    }
}
