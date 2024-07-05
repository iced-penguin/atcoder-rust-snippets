use cargo_snippet::snippet;

#[snippet("cum_sum")]
/// 1次元ベクタの累積和を取る
pub fn cum_sum<T>(x: &mut Vec<T>)
where
    T: num::Num + Copy,
{
    if x.is_empty() {
        return;
    }
    for i in 0..(x.len() - 1) {
        x[i + 1] = x[i + 1] + x[i];
    }
}

#[snippet(include = "cum_sum")]
#[snippet("cum_sum_2d")]
/// 2次元ベクタの累積和を取る
pub fn cum_sum_2d<T>(x: &mut Vec<Vec<T>>)
where
    T: num::Num + Copy,
{
    if x.is_empty() || x[0].is_empty() {
        return;
    }
    let h = x.len();
    let w = x[0].len();
    // 横方向に累積和を取る
    for xx in x.iter_mut() {
        cum_sum(xx);
    }
    // 縦方向に累積和を取る
    for i in 0..(h - 1) {
        for j in 0..w {
            x[i + 1][j] = x[i + 1][j] + x[i][j];
        }
    }
}

#[test]
fn test_cum_sum() {
    let mut x1: Vec<i64> = vec![1, 2, 3, 4, 5];
    cum_sum(&mut x1);
    assert_eq!(x1, vec![1, 3, 6, 10, 15]);

    let mut x2: Vec<usize> = vec![1];
    cum_sum(&mut x2);
    assert_eq!(x2, vec![1]);

    let mut x3: Vec<i32> = Vec::new();
    cum_sum(&mut x3);
    assert_eq!(x3, vec![]);
}

#[test]
fn test_cum_sum_2d() {
    let mut x1: Vec<Vec<i64>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    cum_sum_2d(&mut x1);
    assert_eq!(x1, vec![vec![1, 3, 6], vec![5, 12, 21], vec![12, 27, 45],]);

    let mut x2: Vec<Vec<usize>> = vec![vec![1, 2, 3]];
    cum_sum_2d(&mut x2);
    assert_eq!(x2, vec![vec![1, 3, 6]]);

    let mut x3: Vec<Vec<i32>> = vec![vec![]];
    cum_sum_2d(&mut x3);
    assert_eq!(x3, vec![vec![]]);

    let mut x4: Vec<Vec<i32>> = vec![];
    cum_sum_2d(&mut x4);
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(x4, expected);
}
