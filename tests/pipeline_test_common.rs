pub fn check_result<T: serde::Serialize>(value: T, expected: &str) {
    let result = serde_json::to_string(&value).unwrap();
    assert_eq!(
        expected, result,
        "\nExpected: {}\nGot:      {}",
        expected, result
    );
}
