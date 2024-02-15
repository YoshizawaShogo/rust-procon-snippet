use cargo_snippet::snippet;


#[snippet("binary_indexed_tree")]
/// 遅延セグメント木よりも機能が少ないが、
/// 使いやすく、高速である事も多い
struct BinaryIndexedTree<A>(Vec<A>)
where
    A: Sized + std::ops::AddAssign + Clone + Default;

impl<A> FromIterator<A> for BinaryIndexedTree<A>
where
    A: Sized + std::ops::AddAssign + Clone + Default,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[snippet("binary_indexed_tree")]
impl<A> BinaryIndexedTree<A>
where
    A: Sized + std::ops::AddAssign + Clone + Default + std::ops::Sub<Output = A>,
{
    /// i番目の要素にaddendを加算する
    fn add(&mut self, mut i: usize, addend: A) {
        debug_assert!(i < self.0.len());
        i += 1;
        while i <= self.0.len() {
            self.0[i - 1] += addend.clone();
            i += lsb(i);
        }
    }
    /// [0, r) の合計を求める
    fn sum_0_to_r(&self, r: usize) -> A {
        debug_assert!(r <= self.0.len());
        let mut i = r;
        let mut sum = A::default();
        while i != 0 {
            sum += self.0[i - 1].clone();
            i -= lsb(i);
        }
        sum
    }
    /// [l, r) の合計値を取得
    fn sum_l_to_r(&self, l: usize, r: usize) -> A {
        debug_assert!(l <= r);
        debug_assert!(r <= self.0.len());
        self.sum_0_to_r(r) - self.sum_0_to_r(l)
    }
}

#[snippet("binary_indexed_tree")]
fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}