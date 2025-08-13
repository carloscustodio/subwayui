//! Utilities.
pub fn px(v: impl Into<i32>) -> String {
    format!("{}px", v.into())
}
