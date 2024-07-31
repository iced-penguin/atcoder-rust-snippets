use cargo_snippet::snippet;

#[snippet]
/// 辞書順で次の順列を求める. 次がない場合はfalseを返す
pub fn next_permutaion<T>(v: &mut [T]) -> bool
where
    T: PartialOrd,
{
    // v[i] < v[i+1] を満たす最大のiを求める
    let Some(i) = v.windows(2).rposition(|x| x[0] < x[1]) else {
        return false;
    };
    // v[i] < v[j] を満たす最大のjを求める
    let Some(j) = v.iter().rposition(|x| v[i] < *x) else {
        return false;
    };
    v.swap(i, j);
    v[(i + 1)..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutations() {
        // 元ベクタ, 次があるか, 次のベクタ
        let cases = vec![
            // 全ての順列が辞書順に列挙されること
            (vec![1, 2, 3], true, vec![1, 3, 2]),
            (vec![1, 3, 2], true, vec![2, 1, 3]),
            (vec![2, 1, 3], true, vec![2, 3, 1]),
            (vec![2, 3, 1], true, vec![3, 1, 2]),
            (vec![3, 1, 2], true, vec![3, 2, 1]),
            (vec![3, 2, 1], false, vec![3, 2, 1]),
            // 要素が1つだけの場合
            (vec![1], false, vec![1]),
            // 空の場合
            (vec![], false, vec![]),
        ];
        for (mut src, has_next, dst) in cases {
            assert_eq!(next_permutaion(&mut src), has_next);
            assert_eq!(src, dst);
        }
    }
}
