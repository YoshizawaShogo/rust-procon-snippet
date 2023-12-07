use cargo_snippet::snippet;

#[snippet("inversion_number")]
fn inversion_number(array: Vec<usize>) -> usize
{
    struct BinaryIndexedTree<A>(Vec<A>)
    where
        A: Sized + std::ops::AddAssign + Clone + Default;
    impl<A> BinaryIndexedTree<A>
    where
        A: Sized + std::ops::AddAssign + Clone + Default,
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
    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }

    // 
    let mut bit = BinaryIndexedTree(vec![0; *array.iter().max().unwrap() + 1]);
    let mut sum = 0;

    for (i, &e) in array.iter().enumerate() {
        sum += i - bit.sum(e);
        bit.add(e, 1);
    }

    sum
}