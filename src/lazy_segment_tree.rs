use cargo_snippet::snippet;

#[snippet("lazy_segment_tree")]
/// [l, r)
/// to edit
/// 値
type T = isize;

#[snippet("lazy_segment_tree")]
/// to edit
/// 遅延値
type L = isize;

#[snippet("lazy_segment_tree")]
/// to edit
/// 値の初期値
fn identity_element() -> T {
    0
}

#[snippet("lazy_segment_tree")]
/// to edit
/// 子の値をマージして親の値を求める関数
fn merge_value(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

#[snippet("lazy_segment_tree")]
/// to edit
/// 遅延値を値に適用させる関数
fn commit_lazy(a: T, b: Option<L>) -> T {
    if let Some(b) = b {
        a + b
    } else {
        a
    }
}

#[snippet("lazy_segment_tree")]
/// to edit
/// 二つの遅延値をマージして一つの遅延値にする関数
fn merge_lazy(a: Option<L>, b: Option<L>) -> Option<L> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

#[snippet("lazy_segment_tree")]
/// [l, r)
#[derive(Debug, PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
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
/// data: 値、Noneの時はDirty、葉はDirtyにならない
/// lazy: 遅延値
/// update(): 値を適用
/// query(): 値を取得
#[derive(Debug)]
struct LazySegmentTree {
    r_edge: usize,
    value: Vec<Option<T>>,
    lazy_value: Vec<Option<L>>,
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
            value: data,
            lazy_value: vec![None; n],
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
        let lazy_sum = merge_lazy(lazy_value, self.lazy_value[current_i]);

        // 範囲外の場合、遅延値のみ処理
        // 値は触らない
        if target.excludes(&current_range) {
            self.lazy_value[current_i] = lazy_sum;
            return None;
        }
        
        // 範囲内の場合、
        if target.contains(&current_range) {
            // 値を持つ場合は、子が何であろうと関係ない
            if let Some(data) = self.value[current_i] {
                self.value[current_i] = Some(commit_lazy(data, lazy_sum));
                self.lazy_value[current_i] = None;

                // 葉で無いならば、遅延値を伝播
                if current_range.len() != 1 {
                    self.lazy_value[current_i * 2 + 1] =
                        merge_lazy(self.lazy_value[current_i * 2 + 1], lazy_sum);
                    self.lazy_value[current_i * 2 + 2] =
                        merge_lazy(self.lazy_value[current_i * 2 + 2], lazy_sum);
                }
                return self.value[current_i];
            }
            // 値を持たない場合は、先に子の値を評価
            else {
                let (child_l, child_r) = current_range.spilit_at_mid();
                let a = self._query(target, lazy_sum, current_i * 2 + 1, child_l);
                let b = self._query(target, lazy_sum, current_i * 2 + 2, child_r);
                self.value[current_i] = merge_value(a, b);
                return self.value[current_i];
            }
        }
        
        // 範囲外でも範囲内でもなく、重なっている場合は子の値に依存する
        // 遅延値はNoneにする
        self.lazy_value[current_i] = None;
        let (child_l, child_r) = current_range.spilit_at_mid();
        let a = self._query(target, lazy_sum, current_i * 2 + 1, child_l);
        let b = self._query(target, lazy_sum, current_i * 2 + 2, child_r);
        // ここで値を更新してはいけない
        return merge_value(a, b);
    }
    /// target[l, r)に対してvalueを適用
    fn update(&mut self, target: &Range, lazy_value: L) {
        let valid_range = Range::new(0, self.r_edge);
        debug_assert!(valid_range.contains(target));
        self._update(target, lazy_value, None, 0, valid_range);
    }
    /// valueとpropagatedは適用範囲が異なるため、両方必要
    /// propagatedは無条件に親から子へ適用
    /// valueはtargetに含まれている時のみ適用
    /// 全部遅延させる。評価はqueryの際に行う
    fn _update(
        &mut self,
        target: &Range,
        lazy_value: L,
        propagated: Option<L>,
        current_i: usize,
        current_range: Range,
    ) {
        let lazy_sum = merge_lazy(self.lazy_value[current_i], propagated);
        // 範囲外ならばpropagatedのみ反映
        if target.excludes(&current_range) {
            self.lazy_value[current_i] = lazy_sum;
            return;
        }
        // 範囲内ならばvalueとpropagatedを反映
        if target.contains(&current_range) {
            let lazy_sum = merge_lazy(lazy_sum, Some(lazy_value));
            self.lazy_value[current_i] = lazy_sum;
            return;
        }

        // 範囲外でも範囲内でもなく、重なっている場合は子の値に依存する
        // 値はDirtyにする
        self.lazy_value[current_i] = None;
        self.value[current_i] = None;
        let (child_l, child_r) = current_range.spilit_at_mid();
        self._update(target, lazy_value, lazy_sum, current_i * 2 + 1, child_l);
        self._update(target, lazy_value, lazy_sum, current_i * 2 + 2, child_r);
    }
}