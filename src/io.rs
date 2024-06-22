use cargo_snippet::snippet;

#[snippet]
pub fn read<T>() -> T
where
    T: std::str::FromStr + Default,
{
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    T::from_str(&s.trim()).unwrap_or_default()
}

#[snippet]
pub fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr + Default,
{
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    s.trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap_or_default())
        .collect()
}

#[snippet]
pub fn take2<T>(v: &Vec<T>) -> (T, T)
where
    T: Copy,
{
    (v[0], v[1])
}

#[snippet]
pub fn take3<T>(v: &Vec<T>) -> (T, T, T)
where
    T: Copy,
{
    (v[0], v[1], v[2])
}

#[snippet]
pub fn take4<T>(v: &Vec<T>) -> (T, T, T, T)
where
    T: Copy,
{
    (v[0], v[1], v[2], v[3])
}
