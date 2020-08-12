pub fn foo() -> i32 {
    42
}

pub fn bar(numbers: &[u32]) -> u32 {
    let mut res: u32 = 0;
    for i in 0..numbers.len() {
        res += numbers[i];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn new_test() {
        assert!("Hello World!".contains("World"));
    }

    #[test]
    fn foo_test() {
        assert_eq!(foo(), 42);
    }
}
