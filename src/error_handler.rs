use diesel::result::Error as DieselError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                CustomError::new(404, "The employee record not found".to_string())
            }
            err => CustomError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}
