use cargo_snippet::snippet;

#[snippet("lazy_segment_tree")]
/// to edit
type T = isize;

#[snippet("lazy_segment_tree")]
/// to edit
type L = isize;

#[snippet("lazy_segment_tree")]
/// to edit
fn identity_element() -> T {
    0
}

#[snippet("lazy_segment_tree")]
/// to edit
fn merge_data(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => {
            Some(a.max(b))
        },
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

#[snippet("lazy_segment_tree")]
/// to edit
fn commit_lazy(a: T, b: L) -> T {
    a + b
}

#[snippet("lazy_segment_tree")]
/// to edit
fn merge_lazy(a: Option<L>, b: Option<L>) -> Option<L> {
    match (a, b) {
        (Some(a), Some(b)) => {
            Some(a + b)
        },
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

#[snippet("lazy_segment_tree")]
/// [l, r)
#[derive(PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
struct Range {
    l: usize,
    r: usize,
}

#[snippet("lazy_segment_tree")]
impl Range {
    fn new(l: usize, r: usize) -> Self {
        debug_assert!(l < r);
        Range { l, r }
    }
    fn len(&self) -> usize {
        self.r - self.l
    }
    fn spilit_at_mid(self) -> (Range, Range) {
        (
            Range::new(self.l, (self.l + self.r + 1) / 2),
            Range::new((self.l + self.r + 1) / 2, self.r),
        )
    }
    fn contains(&self, &target: &Range) -> bool {
        if self.l <= target.l && target.r <= self.r {
            true
        } else {
            false
        }
    }
    fn excludes(&self, &target: &Range) -> bool {
        if target.r <= self.l || self.r <= target.l {
            true
        } else {
            false
        }
    }
}

#[snippet("lazy_segment_tree")]
struct LazySegmentTree {
    r_edge: usize,
    data: Vec<Option<T>>,
    lazy: Vec<Option<L>>,
}

#[snippet("lazy_segment_tree")]
impl LazySegmentTree {
    fn new(size: usize) -> Self {
        let mut i = 1;
        while i < size {
            i *= 2;
        }
        let n = 2 * i - 1;
        let mut data = vec![None; n - i];
        data.append(&mut vec![Some(identity_element()); i]);
        LazySegmentTree {
            r_edge: i,
            data: data,
            lazy: vec![None; n],
        }
    }
    fn query(&mut self, target: &Range) -> T {
        let valid_range = Range::new(0, self.r_edge);
        debug_assert!(valid_range.contains(target));
        self._query(target, None, 0, valid_range).unwrap()
    }
    fn _query(
        &mut self,
        target: &Range,
        lazy_value: Option<L>,
        current_i: usize,
        current_range: Range,
    ) -> Option<T> {
        if target.excludes(&current_range) {
            if let Some(lazy_sum) = merge_lazy(lazy_value, self.lazy[current_i]) {
                self.data[current_i] = None;
                self.lazy[current_i] = Some(lazy_sum);
            }
            return None;
        } else if target.contains(&current_range) {
            if let Some(data) = self.data[current_i] {
                if let Some(lazy_sum) = merge_lazy(lazy_value, self.lazy[current_i]) {
                    self.data[current_i] = Some(commit_lazy(data, lazy_sum));
                    self.lazy[current_i] = None;
                    if current_range.len() != 1 {
                        self.lazy[current_i * 2 + 1] =
                            merge_lazy(self.lazy[current_i * 2 + 1], Some(lazy_sum));
                        self.lazy[current_i * 2 + 2] =
                            merge_lazy(self.lazy[current_i * 2 + 2], Some(lazy_sum));
                    }
                }
                return self.data[current_i];
            } else {
                let lazy_sum = merge_lazy(lazy_value, self.lazy[current_i]);
                self.lazy[current_i] = None;

                if current_range.len() == 1 {
                    self.data[current_i] = Some(commit_lazy(identity_element(), lazy_sum.unwrap()));
                    return self.data[current_i];
                } else {
                    let (child_l, child_r) = current_range.spilit_at_mid();
                    let a = self._query(target, lazy_sum, current_i * 2 + 1, child_l);
                    let b = self._query(target, lazy_sum, current_i * 2 + 2, child_r);
                    self.data[current_i] = merge_data(a, b);
                    return self.data[current_i];
                }
            }
        } else {
            let lazy_sum = merge_lazy(lazy_value, self.lazy[current_i]);
            self.lazy[current_i] = None;
            let (child_l, child_r) = current_range.spilit_at_mid();
            let a = self._query(target, lazy_sum, current_i * 2 + 1, child_l);
            let b = self._query(target, lazy_sum, current_i * 2 + 2, child_r);
            return merge_data(a, b);
        };
    }
    fn update(&mut self, target: &Range, value: L) {
        let valid_range = Range::new(0, self.r_edge);
        debug_assert!(valid_range.contains(target));
        self._update(target, value, None, 0, valid_range);
    }
    fn _update(
        &mut self,
        target: &Range,
        value: L,
        propagated: Option<L>,
        current_i: usize,
        current_range: Range,
    ) {
        let lazy_sum = merge_lazy(self.lazy[current_i], propagated);
        if target.excludes(&current_range) {
            self.lazy[current_i] = lazy_sum;
            return;
        } else if target.contains(&current_range) {
            let lazy_sum = merge_lazy(lazy_sum, Some(value));
            self.lazy[current_i] = lazy_sum;
            return;
        } else {
            self.lazy[current_i] = None;
            self.data[current_i] = None;
            let (child_l, child_r) = current_range.spilit_at_mid();
            self._update(target, value, lazy_sum, current_i * 2 + 1, child_l);
            self._update(target, value, lazy_sum, current_i * 2 + 2, child_r);
        }
    }
}