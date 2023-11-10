use cargo_snippet::snippet;

#[snippet("mod_int")]
#[derive(Clone)]
struct ModInt {
    value: usize,
    modulus: usize,
    inv_div: std::collections::HashMap<usize, usize>,
}

#[snippet("mod_int")]
impl ModInt {
    fn new(modulus: usize, init: usize) -> Self {
        Self {
            value: init,
            modulus: modulus,
            inv_div: std::collections::HashMap::new(),
        }
    }
    #[allow(unused)]
    fn add(mut self, n: usize) -> Self {
        let n = n % self.modulus;
        self.value += n;
        self.value %= self.modulus;
        self
    }
    #[allow(unused)]
    fn sub(mut self, n: usize) -> Self {
        let n = n % self.modulus;
        if self.value < n {
            self.value += self.modulus - n;
        } else {
            self.value -= n;
        }
        self
    }
    #[allow(unused)]
    fn mul(mut self, n: usize) -> Self {
        let n = n % self.modulus;
        self.value *= n;
        self.value %= self.modulus;
        self
    }
    #[allow(unused)]
    fn div(mut self, n: usize) -> Self {
        let n = n % self.modulus;
        let inv_n = if let Some(inv_n) = self.inv_div.get(&n) {
            *inv_n
        } else {
            let inv_n = mod_inv(n, self.modulus);
            self.inv_div.insert(n, inv_n);
            inv_n
        };
        self.value *= inv_n;
        self.value %= self.modulus;
        self
    }
}

#[snippet("mod_int")]
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
