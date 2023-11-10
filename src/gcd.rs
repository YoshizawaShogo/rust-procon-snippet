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
