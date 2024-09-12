use std::fmt::Display;

#[allow(dead_code)]
pub(crate) fn display_option<T: Display>(option: &Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "".to_string(),
    }
}
