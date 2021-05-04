
#[cfg(test)]
mod tests {
    use search::preprocessing::tokenize;
    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("Hello World".to_string()), vec!["Hello", "World"])
    }
}
