use cargo_snippet::snippet;

#[snippet("JoinToString")]
pub trait JoinToString {
    /// イテレータの要素を連結して文字列にする
    fn join_to_string(self, sep: &str) -> String
    where
        Self: Sized,
        Self: Iterator,
        Self::Item: std::string::ToString,
    {
        self.map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}

#[snippet("JoinToString")]
impl<I: Iterator> JoinToString for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_to_string() {
        // ベクタ
        // 文字列
        assert_eq!(vec!["a", "b", "c"].iter().join_to_string(" "), "a b c");
        // 数値
        assert_eq!(vec![0, 1, 2].iter().join_to_string("."), "0.1.2");
        // 空ベクタ
        assert_eq!(Vec::<i32>::new().iter().join_to_string(","), "");
        // 要素数1
        assert_eq!(vec!["a"].iter().join_to_string(" "), "a");

        // 配列
        let arr = [0, 1, 2, 3];
        assert_eq!(arr.iter().join_to_string(" "), "0 1 2 3");
        // スライス
        let slice = &arr[1..];
        assert_eq!(slice.iter().join_to_string(" "), "1 2 3");
    }
}
