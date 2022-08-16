fn float_sort<T : PartialOrd>(data: &mut [T]) {
    use std::cmp::Ordering::Less;
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
}