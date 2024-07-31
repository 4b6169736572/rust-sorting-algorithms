#[cfg(test)]
mod tests {

    use crate::{
        bubble_sort::bubble_sort, insertion_sort::insertion_sort, selection_sort::selection_sort,
    };

    // Bubble sort
    #[test]
    fn bubble_two_one() {
        let mut input = [2, 1];
        bubble_sort(&mut input);
        assert_eq!(input, [1, 2]);
    }

    #[test]
    fn bubble_reversed() {
        let mut input = [5, 4, 3, 2, 1];
        bubble_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_equal() {
        let mut input = [2, 4, 2, 1, 3, 4];
        bubble_sort(&mut input);
        assert_eq!(input, [1, 2, 2, 3, 4, 4]);
    }
    #[test]
    fn bubble_one() {
        let mut input = [1];
        bubble_sort(&mut input);
        assert_eq!(input, [1]);
    }

    // Selection sort
    #[test]
    fn selection_sort_two_one() {
        let mut input = [2, 1];
        selection_sort(&mut input);
        assert_eq!(input, [1, 2]);
    }

    #[test]
    fn selection_sort_reversed() {
        let mut input = [5, 4, 3, 2, 1];
        selection_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_sort_equal() {
        let mut input = [1, 1, 1, 1, 1];
        selection_sort(&mut input);
        assert_eq!(input, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn selection_sort_one() {
        let mut input = [1];
        selection_sort(&mut input);
        assert_eq!(input, [1]);
    }

    // Insertion sort
    #[test]
    fn insertion_sort_two_one() {
        let mut input = [2, 1];
        insertion_sort(&mut input);
        assert_eq!(input, [1, 2]);
    }

    #[test]
    fn insertion_sort_reversed() {
        let mut input = [5, 4, 3, 2, 1];
        insertion_sort(&mut input);
        assert_eq!(input, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_sort_equal() {
        let mut input = [1, 1, 1, 1, 1];
        insertion_sort(&mut input);
        assert_eq!(input, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn insertion_sort_one() {
        let mut input = [1];
        insertion_sort(&mut input);
        assert_eq!(input, [1]);
    }
}
