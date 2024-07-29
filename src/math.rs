use cargo_snippet::snippet;

#[snippet]
/// a, b の最大公約数を求める（Euclidの互除法）
pub fn gcd<T>(a: T, b: T) -> T
where
    T: num::Unsigned + Copy,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet(include = "gcd")]
/// a, b の最小公倍数を求める
pub fn lcm<T>(a: T, b: T) -> T
where
    T: num::Unsigned + Copy,
{
    a * b / gcd(a, b)
}

#[snippet]
/// 最大値
pub fn max<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

#[snippet]
/// 最小値
pub fn min<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

#[snippet]
/// 絶対値
pub fn abs<T>(a: T) -> T
where
    T: num::Signed,
{
    a.abs()
}

#[snippet]
/// baseを基数としてnの各桁の数の総和を求める
pub fn sum_digits<T>(n: T, base: T) -> T
where
    T: num::Signed + Copy + PartialOrd,
{
    if base <= T::one() {
        panic!("base must be at least two")
    }

    let zero = T::zero();
    let mut num = if n > zero { n } else { -n };
    let mut sum = zero.clone();

    while num > zero {
        sum = sum + (num % base);
        num = num / base;
    }
    sum
}

#[snippet]
/// 基数base、指数expの冪乗をmで割った余りを求める（繰り返し二乗法）
pub fn pow_mod<T>(base: T, exp: T, m: T) -> T
where
    T: num::PrimInt,
{
    // 負の基数と指数をサポートしない
    if base < T::zero() {
        panic!("base must not be negative")
    }
    if exp < T::zero() {
        panic!("exp must not be negative")
    }
    let mut b = base % m;
    let mut n = exp;
    let mut pow = T::one();
    while n > T::zero() {
        if !(n & T::one()).is_zero() {
            pow = (pow * b) % m;
        }
        b = (b * b) % m;
        n = n >> 1;
    }
    pow
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_max() {
        assert_eq!(max(5i32, 10i32), 10);
        assert_eq!(max(10i32, 5i32), 10);
        assert_eq!(max(5u32, 10u32), 10);
        assert_eq!(max(5f32, 10f32), 10.0);
        assert_eq!(max(5, -5), 5);
        assert_eq!(max(5, 0), 5);
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

    #[test]
    fn test_abs() {
        assert_eq!(abs(3), 3);
        assert_eq!(abs(-3), 3);
        assert_eq!(abs(0), 0);
        assert_eq!(abs(3.0), 3.0);
    }

    #[test]
    fn test_sum_digits() {
        // 10進数
        assert_eq!(sum_digits(12345, 10), 15);
        // 2進数
        assert_eq!(sum_digits(12345, 2), 6);
        // 0
        assert_eq!(sum_digits(0, 10), 0);
        // 負の数
        assert_eq!(sum_digits(-123, 10), 6);
    }

    #[test]
    #[should_panic]
    fn test_sum_digits_panic() {
        sum_digits(123, 1);
        sum_digits(123, 0);
        sum_digits(123, -1);
    }

    #[test]
    fn test_pow_mod() {
        // 基数が0
        assert_eq!(pow_mod(0, 3, 5), 0);
        // 基数が正
        assert_eq!(pow_mod(2, 3, 5), 3);
        // 指数が0
        assert_eq!(pow_mod(2, 0, 5), 1);
        // 指数が1
        assert_eq!(pow_mod(2, 1, 5), 2);
        // 基数が大きい
        assert_eq!(pow_mod(1_000_000_000i64, 10i64, 1000000007i64), 282475249);
        // 指数が大きい
        assert_eq!(
            pow_mod(10i64, 1_000_000_000i64, 1000000007i64),
            142857001i64
        );
    }

    #[test]
    #[should_panic]
    fn test_pow_mod_panic() {
        // 基数が負
        pow_mod(-2, 3, 5);
        // 指数が負
        pow_mod(2, -3, 5);
    }
}
