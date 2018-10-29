pub use super::{ErrorInformation, Result};

#[macro_export]
macro_rules! method_success {
    ($a: expr) => {
        rocket_contrib::Json($crate::Result {
            error: false,
            error_info: None,
            result: Some($a),
        })
    };
}

#[macro_export]
macro_rules! method_error {
    (code = $a: expr, details = $b: expr) => {
        rocket_contrib::Json($crate::Result {
            error: true,
            error_info: Some($crate::ErrorInformation {
                description: $b.to_string(),
                error_code: $a.to_string(),
            }),
            result: None,
        })
    };
}
