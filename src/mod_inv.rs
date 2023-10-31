use cargo_snippet::snippet;

#[snippet("MOD_INV")]
fn mod_inv(divisor: usize, modulus: usize) -> usize {
    // 1/d ≡ inv (mod m)
    // 拡張ユークリッドの互除法
    // d*inv + m*x == GCD(d, m) (xは任意の整数)

    let mut a = divisor;
    let mut b = modulus;

    let mut inv: isize = 1;
    let mut x: isize = 0;

    while b != 0 {
        (inv, x) = (x, inv - (a / b) as isize * x);
        (a, b) = (b, a % b);
    }

    if inv < 0 {
        (inv + modulus as isize) as usize
    } else {
        inv as usize
    }
}
