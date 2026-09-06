use std::cmp::Ordering;

pub fn compare(a: f64, b: f64) -> Ordering {
    debug_assert!(!f64::is_nan(a));
    debug_assert!(!f64::is_nan(b));

    unsafe { a.partial_cmp(&b).unwrap_unchecked() }
}
