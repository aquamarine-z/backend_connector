

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn it_works() {
        let backend=Backend::new("127.0.0.1".to_string(), 8080);
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
