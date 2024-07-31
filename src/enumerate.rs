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
        // 全ての順列が辞書順に列挙されること
        let case1 = vec![
            (vec![1, 2, 3], true, vec![1, 3, 2]),
            (vec![1, 3, 2], true, vec![2, 1, 3]),
            (vec![2, 1, 3], true, vec![2, 3, 1]),
            (vec![2, 3, 1], true, vec![3, 1, 2]),
            (vec![3, 1, 2], true, vec![3, 2, 1]),
            (vec![3, 2, 1], false, vec![3, 2, 1]),
        ];
        for (mut src, has_next, dst) in case1 {
            assert_eq!(next_permutaion(&mut src), has_next);
            assert_eq!(src, dst);
        }
        // 重複する要素がある場合、重複した順列を除いて列挙されること
        let case2 = vec![
            (vec!['a', 'a', 'b'], true, vec!['a', 'b', 'a']),
            (vec!['a', 'b', 'a'], true, vec!['b', 'a', 'a']),
            (vec!['b', 'a', 'a'], false, vec!['b', 'a', 'a']),
        ];
        for (mut src, has_next, dst) in case2 {
            assert_eq!(next_permutaion(&mut src), has_next);
            assert_eq!(src, dst);
        }
        // 要素が1つだけの場合
        assert_eq!(next_permutaion(&mut vec![1]), false);
        // 空の場合
        assert_eq!(next_permutaion(&mut Vec::<i32>::new()), false);
    }
}
