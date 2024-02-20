use cargo_snippet::snippet;

#[snippet("mod_inv")]
/// n/d ≡ n*inv (mod m). (nは任意の整数)
/// 拡張ユークリッドの互除法を用いた。
/// d*inv + m*x == GCD(d, m). (xは任意の整数)
/// Usage: let inv = mod_inv(d, m);
/// ただし、dとmは互いに素である必要がある
fn mod_inv(divisor: usize, modulus: usize) -> usize {
    debug_assert_ne!(divisor, 0);
    debug_assert_ne!(modulus, 0);
    
    let mut a = divisor;
    let mut b = modulus;

    let mut inv: isize = 1;
    let mut x: isize = 0;

    while b != 0 {
        (inv, x) = (x, inv - (a / b) as isize * x);
        (a, b) = (b, a % b);
    }

    debug_assert_eq!(a, 1);

    if inv < 0 {
        (inv + modulus as isize) as usize
    } else {
        inv as usize
    }
}