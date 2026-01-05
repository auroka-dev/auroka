#[macro_export]
macro_rules! assert_eq_diff {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_str = format!("{}", left_val);
                    let right_str = format!("{}", right_val);

                    let diff = $crate::compute_diff(&left_str, &right_str);

                    panic!("assertion failed: `(left == right)`\nleft: `{}`\nright: `{}`\n\n{}", left_str, right_str, diff)
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let left_str = format!("{}", left_val);
                    let right_str = format!("{}", right_val);

                    let diff = $crate::compute_diff(&left_str, &right_str);

                    panic!("{}\n\n{}", format_args!($($arg)+), diff)
                }
            }
        }
    });
}
