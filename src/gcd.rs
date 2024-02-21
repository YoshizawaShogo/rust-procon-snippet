use cargo_snippet::snippet;

#[snippet("gcd")]
fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + PartialEq + std::ops::Rem<Output = T> + std::ops::Sub<Output = T> + From<u8>,
{
    let zero: T = 0.into();
    while b != zero {
        (a, b) = (b, a % b);
    }
    a
}

#[snippet("ext_euclid")]
/// 拡張ユークリッド互除法
/// g := gcd(a, b);
/// a*x + b*y == gを満たす組(x, y)が少なくとも一つは存在する。
/// Usage: (g, x, y) = ext_euclid(a, b);
fn ext_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let origin_a = a;
    let origin_b = b;
    debug_assert_ne!(origin_a, 0);
    debug_assert_ne!(origin_b, 0);

    let mut a = a;
    let mut b = b;
    let mut x = 1;
    let mut y = 0;

    if origin_a < origin_b {
        (a, b) = (b, a);
    }

    while b != 0 {
        (x, y) = (y, x - (a / b) * y);
        (a, b) = (b, a % b);
    }

    if origin_a < origin_b {
        (x, y) = (y, x);
    }
    
    (a, x, y)
}