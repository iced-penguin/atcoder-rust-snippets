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

#[snippet]
/// a, b の最大公約数を求める（Euclidの互除法）
pub fn gcd<T>(a: T, b: T) -> T
where T: num::Unsigned + Copy
{
    if b == T::zero() { a } else { gcd(b, a % b) }
}

#[test]
fn test_gcd() {
    // u8, a > b
    assert_eq!(gcd(10u8, 5u8), 5);
    // u16, a < b
    assert_eq!(gcd(14u16, 21u16), 7);
    // u32, a = b
    assert_eq!(gcd(100u32, 100u32), 100);
    // u64, b = 1
    assert_eq!(gcd(5u64, 1u64), 1);
    // u128, a = 1
    assert_eq!(gcd(1u128, 5u128), 1);
    // usize, 互いに素
    assert_eq!(gcd(17usize, 23usize), 1);
}

#[snippet(include = "gcd")]
/// a, b の最小公倍数を求める
pub fn lcm<T>(a: T, b: T) -> T 
where T: num::Unsigned + Copy
{
    a * b / gcd(a, b)
}

#[test]
fn test_lcm() {
    // u8, a > b
    assert_eq!(lcm(10u8, 5u8), 10);
    // u16, a < b
    assert_eq!(lcm(5u16, 10u16), 10);
    // u32, a = b
    assert_eq!(lcm(10u32, 10u32), 10);
    // u64, b = 1
    assert_eq!(lcm(14u64, 1u64), 14);
    // u128, a = 1
    assert_eq!(lcm(1u128, 17u128), 17);
    // usize, 互いに素
    assert_eq!(lcm(17usize, 23usize), 17 * 23);
}

#[snippet]
pub fn max<T>(a: T, b: T) -> T 
where T: std::cmp::PartialOrd
{
    if a > b { a } else { b }
}

#[test]
fn test_max() {
    assert_eq!(max(5i32, 10i32), 10);
    assert_eq!(max(10i32, 5i32), 10);
    assert_eq!(max(5u32, 10u32), 10);
    assert_eq!(max(5f32, 10f32), 10.0);
    assert_eq!(max(5, -5), 5);
    assert_eq!(max(5, 0), 5);
}

#[snippet]
pub fn min<T>(a: T, b: T) -> T
where T: std::cmp::PartialOrd
{
    if a < b { a } else { b }
}

#[test]
fn test_min() {
    assert_eq!(min(5i32, 10i32), 5);
    assert_eq!(min(10i32, 5i32), 5);
    assert_eq!(min(5u32, 10u32), 5);
    assert_eq!(min(5f32, 10f32), 5.0);
    assert_eq!(min(5, -5), -5);
    assert_eq!(min(5, 0), 0);
}