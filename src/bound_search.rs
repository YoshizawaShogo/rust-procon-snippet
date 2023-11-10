use cargo_snippet::snippet;

#[snippet("bound_search")]
fn bound_search<T>(mut met: T, mut violated: T, f: impl Fn(T) -> bool) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Sub<Output = T>
        + PartialOrd
        + From<u8>,
{
    debug_assert!(f(met));
    debug_assert!(!f(violated));

    let two = T::from(2);
    while (met > violated && met - violated >= two) || (violated > met && violated - met >= two) {
        let mid = (met + violated) / two;
        if f(mid) {
            met = mid;
        } else {
            violated = mid;
        }
    }
    met
}

#[snippet("bound_search_in_array")]
fn bound_search_in_array<T: Clone>(
    mut met: usize,
    mut violated: usize,
    array: &[T],
    evaluate_fn: impl Fn(T) -> bool,
) -> usize {
    debug_assert!(array.get(met).is_some());
    debug_assert!(evaluate_fn(array[met].clone()));
    debug_assert!(array.get(violated).is_none() || !evaluate_fn(array[violated].clone()));

    while met + 2 <= violated || violated + 2 <= met {
        let mid = (met + violated) / 2;
        let mid_value = array[mid].clone();
        if evaluate_fn(mid_value) {
            met = mid;
        } else {
            violated = mid;
        }
    }
    met
}
