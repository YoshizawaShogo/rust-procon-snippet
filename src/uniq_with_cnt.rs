use cargo_snippet::snippet;

#[snippet("uniq_with_cnt")]
#[allow(unused_imports)]
use std::collections::HashMap;

#[snippet("uniq_with_cnt")]
fn uniq_with_cnt<T>(sorted: Vec<T>) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash,
{
    let mut counts= HashMap::new();
    for e in sorted {
        *counts.entry(e).or_insert(0) += 1;
    }
    counts
}