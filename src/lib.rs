pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_positiv_numbers() {
        assert_eq!(addition(1, 1), 2);
    }
    #[test]
    fn test_add_negative_numbers(){
        assert_eq!(addition(-1, -1), -2);
    }
    #[test]
    fn test_add_zero(){
        assert_eq!(addition(1, 0), 1);
    }
}