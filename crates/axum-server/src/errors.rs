use std::fmt;

use axum::{response::{Response, IntoResponse}, http::{StatusCode, uri::InvalidUri}};
use db::{TokioPostgresError, PoolError};

#[derive(Debug)]
pub enum CustomError {
  FaultySetup(String),
  Database(String),
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
      CustomError::Database(ref cause) => write!(f, "Database Error: {}", cause),
    }
  }
}

impl IntoResponse for CustomError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
      CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
    };

    format!("status = {}, message = {}", status, error_message).into_response()
  }
}

impl From<InvalidUri> for CustomError {
  fn from(err: InvalidUri) -> CustomError {
    CustomError::FaultySetup(err.to_string())
  }
}

impl From<TokioPostgresError> for CustomError {
  fn from(err: TokioPostgresError) -> CustomError {
    CustomError::FaultySetup(err.to_string())
  }
}

impl From<PoolError> for CustomError {
  fn from(err: PoolError) -> CustomError {
    CustomError::FaultySetup(err.to_string())
  }
}
