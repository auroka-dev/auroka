#[macro_export]
macro_rules! assert_not_contains_diff {
    ($haystack:expr, $needle:expr $(,)?) => ({
        match (&$haystack, &$needle) {
            (haystack_val, needle_val) => {
                if haystack_val.contains(needle_val) {
                    let haystack_str = format!("{}", haystack_val);
                    let needle_str = format!("{}", needle_val);

                    let diff = $crate::compute_diff(&haystack_str, &needle_str);

                    panic!("assertion failed: haystack does contain needle\nhaystack:\n{}\nneedle:\n{}\n\n{}", haystack_str, needle_str, diff)
                }
            }
        }
    });
    ($haystack:expr, $needle:expr, $($arg:tt)+) => ({
        match (&$haystack, &$needle) {
            (haystack_val, needle_val) => {
                if haystack_val.contains(needle_val) {
                    let haystack_str = format!("{}", haystack_val);
                    let needle_str = format!("{}", needle_val);

                    let diff = $crate::compute_diff(&haystack_str, &needle_str);

                    panic!("{}\n\n{}", format_args!($($arg)+), diff)
                }
            }
        }
    });
}
