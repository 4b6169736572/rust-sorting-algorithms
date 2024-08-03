#![cfg(test)]

use crate::{
    bubble_sort::bubble_sort, insertion_sort::insertion_sort, quick_sort::quick_sort,
    selection_sort::selection_sort,
};
use paste::paste;

macro_rules! test {
    ($func:ident) => {
        // $func
        paste! {
        #[test]
        fn [<$func _two_one>]() {
            let mut input = [2, 1];
            $func(&mut input);
            assert_eq!(input, [1, 2]);
        }

        #[test]
        fn [<$func _reversed>]() {
            let mut input = [5, 4, 3, 2, 1];
            $func(&mut input);
            assert_eq!(input, [1, 2, 3, 4, 5]);
        }

        #[test]
        fn [<$func _duplicates>]() {
            let mut input = [2, 4, 2, 1, 3, 4];
            $func(&mut input);
            assert_eq!(input, [1, 2, 2, 3, 4, 4]);
        }

        #[test]
        fn [<$func _all_equal>]() {
            let mut input = [2,2,2,2,2];
            $func(&mut input);
            assert_eq!(input, [2,2,2,2,2]);
        }

        #[test]
        fn [<$func _one>]() {
            let mut input = [1];
            $func(&mut input);
            assert_eq!(input, [1]);
        }

        }
    };
}

test!(bubble_sort);
test!(selection_sort);
test!(insertion_sort);
test!(quick_sort);
