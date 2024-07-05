use cargo_snippet::snippet;

#[snippet]
/// [l, r)の範囲でx以上の最初の要素のインデックスを返す. vはソート済みであること.
pub fn lower_bound<T>(v: &Vec<T>, l: usize, r: usize, x: &T) -> usize
where
    T: Ord,
{
    if l >= r {
        return l;
    }
    let m = (l + r) / 2;
    if &v[m] < x {
        lower_bound(v, m + 1, r, x)
    } else {
        lower_bound(v, l, m, x)
    }
}

#[snippet]
/// [l, r)の範囲でxより大きい最初の要素のインデックスを返す. vはソート済みであること.
pub fn upper_bound<T>(v: &Vec<T>, l: usize, r: usize, x: &T) -> usize
where
    T: Ord,
{
    if l >= r {
        return l;
    }
    let m = (l + r) / 2;
    if &v[m] <= x {
        lower_bound(v, m + 1, r, x)
    } else {
        lower_bound(v, l, m, x)
    }
}

#[test]
fn test_lower_bound() {
    let v = vec![1, 3, 5, 7, 9];
    // xと一致する要素あり
    assert_eq!(lower_bound(&v, 0, v.len(), &5), 2);
    // xと一致する要素なし
    assert_eq!(lower_bound(&v, 0, v.len(), &4), 2);
    // xが全要素より大きい
    assert_eq!(lower_bound(&v, 0, v.len(), &11), 5);
    // xが全要素より小さい
    assert_eq!(lower_bound(&v, 0, v.len(), &0), 0);
    // 部分探索 答えが探索範囲を逸脱しないこと（左側）
    assert_eq!(lower_bound(&v, 1, 4, &1), 1);
    // 部分探索 答えが探索範囲を逸脱しないこと（右側）
    assert_eq!(lower_bound(&v, 1, 4, &11), 4);
}

#[test]
fn test_upper_bound() {
    let v = vec![1, 3, 5, 7, 9];
    // xと一致する要素あり
    assert_eq!(upper_bound(&v, 0, v.len(), &5), 3);
    // xと一致する要素なし
    assert_eq!(upper_bound(&v, 0, v.len(), &4), 2);
    // xが全要素より大きい
    assert_eq!(upper_bound(&v, 0, v.len(), &11), 5);
    // xが全要素より小さい
    assert_eq!(upper_bound(&v, 0, v.len(), &0), 0);
    // 部分探索 答えが探索範囲を逸脱しないこと（左側）
    assert_eq!(upper_bound(&v, 1, 4, &0), 1);
    // 部分探索 答えが探索範囲を逸脱しないこと（右側）
    assert_eq!(upper_bound(&v, 1, 4, &11), 4);
}
