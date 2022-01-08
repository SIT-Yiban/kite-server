use num_traits::ToPrimitive;
use poem::error::ResponseError;
use poem::http::StatusCode;

use serde_json::Error as JsonError;
use sqlx::error::Error as SqlxError;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    pub code: u16,
    pub msg: Option<String>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{code:{},msg:\"{:?}\"}}", self.code, self.msg,)
    }
}

impl ResponseError for ApiError {
    fn status(&self) -> StatusCode {
        StatusCode::OK
    }
}

impl ApiError {
    pub fn new<T: ToPrimitive + std::error::Error>(sub_err: T) -> Self {
        Self {
            code: sub_err.to_u16().unwrap(),
            msg: Some(sub_err.to_string()),
        }
    }
}

#[macro_export]
macro_rules! convert_inner_errors {
    ($src_err_type: ident) => {
        impl From<$src_err_type> for ApiError {
            fn from(sub_err: $src_err_type) -> Self {
                Self {
                    code: 1,
                    msg: Some(sub_err.to_string()),
                }
            }
        }
    };
}

convert_inner_errors!(SqlxError);
convert_inner_errors!(JsonError);
