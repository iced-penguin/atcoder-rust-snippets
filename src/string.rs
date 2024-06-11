use cargo_snippet::snippet;

#[snippet]
/// ベクタの要素を連結して文字列にする
pub fn join_to_string<T>(v: &Vec<T>, sep: &str) -> String
where T: std::string::ToString
{
    v.iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

#[test]
fn test_join_to_string() {
    // 文字列
    assert_eq!(join_to_string(&vec!["a", "b", "c"], " "), "a b c");
    // 数値
    assert_eq!(join_to_string(&vec![0, 1, 2], "."), "0.1.2");
    // 空ベクタ
    assert_eq!(join_to_string(&Vec::<i32>::new(), ","), "");
    // 要素数1
    assert_eq!(join_to_string(&vec!["a"], " "), "a");
}