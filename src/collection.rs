use cargo_snippet::snippet;

#[snippet]
/// 回文かどうかを判定する
pub fn is_palindrome<T>(x: &[T]) -> bool
where
    T: PartialEq,
{
    let l = x.len();
    for i in 0..l / 2 {
        if x[i] != x[l - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let cases = vec![
            // 要素数が偶数
            (vec![1, 2, 2, 1], true),
            // 要素数が奇数
            (vec![1, 2, 3, 2, 1], true),
            // 要素数1
            (vec![1], true),
            // 空
            (vec![], true),
            // 回文ではない
            (vec![1, 2, 3, 4], false),
        ];
        for (param, expexted) in cases {
            assert_eq!(is_palindrome(&param), expexted);
        }
    }
}
