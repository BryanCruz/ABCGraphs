pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_equals_four() {
        assert_eq!(4, add(0, 4));
        assert_eq!(4, add(1, 3));
        assert_eq!(4, add(2, 2));
        assert_eq!(4, add(3, 1));
        assert_eq!(4, add(4, 0));
    }

    #[test]
    fn sum_negative_numbers() {
        assert_eq!(0, add(-4, 4));
        assert_eq!(4, add(9, -5));
        assert_eq!(-4, add(-1, -3));
        assert_eq!(-4, add(-2, -2));
        assert_eq!(-4, add(-5, 1));
    }
}
