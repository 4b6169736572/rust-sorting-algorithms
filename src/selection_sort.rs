use std::ptr;

pub fn selection_sort<T: PartialOrd>(input: &mut [T]) {
    let len = input.len();

    if len == 1 {
        return;
    }

    for start in 0..len - 1 {
        // Min assumed to be ith element
        let mut min = start;

        let mut swapped = false;

        // Iterate through unsorted portion and compare with min, if new min found update `min`
        // and set `swapped` to true
        for offset in start + 1..len {
            if input[offset] < input[min] {
                min = offset;
                swapped = true;
            }
        }

        if swapped {
            unsafe {
                ptr::swap(&mut input[min] as *mut T, &mut input[start] as *mut T);
            }
        }
    }
}
