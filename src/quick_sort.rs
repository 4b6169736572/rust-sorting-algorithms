pub fn quick_sort<T: PartialOrd>(input: &mut [T]) {
    let len = input.len();
    if len < 2 {
        return;
    }
    if len == 2 && input[0] > input[1] {
        input.swap(0, 1);
        return;
    } else if len == 2 {
        return;
    }

    let (pivot, rest) = input.split_first_mut().unwrap();

    // Indeces of first and last elements, elements outside of these (further away from the middle)
    // are in the respective side. So left=2 implies f.a. [..2] < pivot.
    let mut left = 0;
    let mut right = rest.len() - 1;

    // Special considerations need to be taken when right == 0, because if right == 0 &&
    // rest[right] > pivot then right == -1, which crashes; in order to handle this pivot must be
    // conditionally swapped with input[right+1] (input[1]) and the latter pivot shifting must be
    // skipped.

    let mut flag = false;

    while left <= right {
        if right == 0 {
            if &rest[0] < pivot {
                input.swap(0, 1);
            }
            flag = true;
            break;
        }
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if !flag {
        input.swap(0, left);
    }

    quick_sort(&mut input[..left]);
    quick_sort(&mut input[right + 1..]);
}
