use std::ptr;

pub fn selection_sort<T: PartialOrd>(input: &mut [T]) {
    let len = input.len();

    for i in 0..len - 1 {
        // Min assumed to be ith element
        let mut min = i;

        let mut swapped = false;

        // Iterate through unsorted portion and compare with min, if new min found update `min`
        // and set `swapped` to true
        for j in i + 1..len {
            if input[j] < input[min] {
                min = j;
                swapped = true;
            }
        }

        if swapped {
            unsafe {
                ptr::swap(&mut input[min] as *mut T, &mut input[i] as *mut T);
            }
        }
    }
}
