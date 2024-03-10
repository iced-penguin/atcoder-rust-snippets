use cargo_snippet::snippet;

#[snippet]
pub fn read<T>() -> T
where T: std::str::FromStr + Default
{
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    T::from_str(&s.trim()).unwrap_or_default()
}

#[snippet]
pub fn read_vec<T>() -> Vec<T>
where T: std::str::FromStr + Default
{
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    s.trim().split_whitespace()
        .map(|c| c.parse().unwrap_or_default())
        .collect()
}

#[snippet]
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