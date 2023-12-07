use cargo_snippet::snippet;

#[snippet("binary_indexed_tree")]
use std::fmt::Debug;

#[snippet("binary_indexed_tree")]
#[derive(Debug)]
struct BinaryIndexedTree<A>(Vec<A>)
where
    A: Sized + std::ops::AddAssign + Clone + Debug + Default;

impl<A> FromIterator<A> for BinaryIndexedTree<A>
where
    A: Sized + std::ops::AddAssign + Clone + Debug + Default,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[snippet("binary_indexed_tree")]
impl<A> BinaryIndexedTree<A>
where
    A: Sized + std::ops::AddAssign + Clone + Debug + Default,
{
    fn add(&mut self, mut i: usize, addend: A) {
        debug_assert!(i < self.0.len());
        i += 1;
        while i <= self.0.len() {
            self.0[i - 1] += addend.clone();
            i += lsb(i);
        }
    }
    fn sum(&self, mut i: usize) -> A {
        debug_assert!(i < self.0.len());
        let mut sum = A::default();
        i += 1;
        while i != 0 {
            sum += self.0[i - 1].clone();
            i -= lsb(i);
        }
        sum
    }
}

#[snippet("binary_indexed_tree")]
fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}