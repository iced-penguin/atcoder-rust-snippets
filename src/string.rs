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

#[snippet("Parenthesis")]
pub struct Parenthesis {
    left_sign: char,
    right_sign: char,
    data: Vec<i8>,
}

#[snippet("Parenthesis")]
impl Parenthesis {
    // TODO 必要が出てきたらジェネリクスにする
    pub fn new(left_sign: char, right_sign: char, data: &Vec<char>) -> Self {
        let d: Vec<i8> = data
            .iter()
            .map(|&x| if x == left_sign { 1 } else { -1 })
            .collect();
        Self {
            left_sign,
            right_sign,
            data: d,
        }
    }

    /// 正しい括弧列かどうか
    pub fn is_valid(&self) -> bool {
        // 正しい括弧列は以下を満たす
        // 1. 左から括弧の数を数えたとき、常に左括弧の数の和が右括弧の数の和以上であること
        // 2. 左括弧と右括弧の数が等しいこと
        let mut sum = 0;
        for d in self.data.iter() {
            sum += d;
            if sum < 0 {
                return false;
            }
        }
        sum == 0
    }

    // TODO 文字列化する時に使用する括弧記号を与えられるようにする
    pub fn string(&self) -> String {
        self.data
            .iter()
            .map(|&x| {
                if x == 1 {
                    self.left_sign
                } else {
                    self.right_sign
                }
            })
            .collect()
    }
}

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

    #[test]
    fn test_parenthesis() {
        let cases = vec![
            // 空文字列
            ("", true),
            // 正しい括弧列
            ("(()())(())", true),
            // 正しくない括弧列 常に左括弧の数の和が右括弧の数の和以上であることを満たさない
            (")))()(((", false),
            // 正しくない括弧列　左括弧と右括弧の数が等しいことを満たさない
            ("(()())(()", false),
        ];
        let lsign = '(';
        let rsign = ')';
        for (input, expected) in cases {
            let data: Vec<char> = input.chars().collect();
            let p = Parenthesis::new(lsign, rsign, &data);
            assert_eq!(p.is_valid(), expected);
        }
    }
}
