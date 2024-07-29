use std::ptr;

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
            unsafe {
                ptr::swap(&mut input[index] as *mut T, &mut input[index + 1] as *mut T);
            }
            swap = true;
        }
        if swap == false {
            break;
        }
    }
}
