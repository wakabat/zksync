// Built-in uses
use std::fmt::{Display, Formatter};

// External uses
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use thiserror::Error;

// Workspace uses

// Local uses
use crate::{api_server::tx_sender::SubmitError, fee_ticker::PriceError};

#[derive(Serialize_repr, Debug, Deserialize)]
#[repr(u16)]
pub enum ErrorCode {
    UnreacheableError = 0,
    CoreApiError = 100,
    TokenZeroPriceError = 200,
    InvalidCurrency = 201,
    InvalidBlockPosition = 202,
    InvalidAccountIdOrAddress = 203,
    AccountNotFound = 204,
    TransactionNotFound = 205,
    StorageError = 300,
    TokenNotFound = 500,
    ExternalApiError = 501,
    InternalError = 600,
    AccountCloseDisabled = 601,
    InvalidParams = 602,
    UnsupportedFastProcessing = 603,
    IncorrectTx = 604,
    TxAddError = 605,
    InappropriateFeeToken = 606,
    CommunicationCoreServer = 607,
    Other = 60_000,
}

/// Error object in a response
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_type: String,
    pub code: ErrorCode,
    pub message: String,
}

/// Trait that can be used to map custom errors to the object.
pub trait ApiError: std::fmt::Display {
    fn error_type(&self) -> String;

    fn code(&self) -> ErrorCode;

    fn message(&self) -> String {
        self.to_string()
    }
}

impl<T> From<T> for Error
where
    T: ApiError,
{
    fn from(t: T) -> Error {
        Error {
            error_type: t.error_type(),
            code: t.code(),
            message: t.message(),
        }
    }
}

impl Error {
    pub fn storage(err: impl Display) -> Error {
        Error::from(StorageError::new(err))
    }

    pub fn core_api(err: impl Display) -> Error {
        Error::from(CoreApiError::new(err))
    }
}

#[derive(Debug)]
pub struct UnreachableError;

impl Display for UnreachableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "Unreachable error; you should never see this message, \
            please contact us at https://github.com/matter-labs/zksync with a report",
        )
    }
}

impl ApiError for UnreachableError {
    fn error_type(&self) -> String {
        String::from("api_error")
    }

    fn code(&self) -> ErrorCode {
        ErrorCode::UnreacheableError
    }
}

#[derive(Error, Debug)]
pub enum InvalidDataError {
    #[error("Cannot show price in zero price token")]
    TokenZeroPriceError,
    #[error("Cannot parse block position. There are only block_number, last_committed, last_finalized options")]
    InvalidBlockPosition,
    #[error("Cannot parse account id or address")]
    InvalidAccountIdOrAddress,
    #[error("Account is not found")]
    AccountNotFound,
    #[error("Cannot parse currency. There are only token_id, usd options")]
    InvalidCurrency,
    #[error("Transaction is not found")]
    TransactionNotFound,
}

impl ApiError for InvalidDataError {
    fn error_type(&self) -> String {
        String::from("invalid_data_error")
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::TokenZeroPriceError => ErrorCode::TokenZeroPriceError,
            Self::InvalidBlockPosition => ErrorCode::InvalidBlockPosition,
            Self::InvalidAccountIdOrAddress => ErrorCode::InvalidAccountIdOrAddress,
            Self::AccountNotFound => ErrorCode::AccountNotFound,
            Self::InvalidCurrency => ErrorCode::InvalidCurrency,
            Self::TransactionNotFound => ErrorCode::TransactionNotFound,
        }
    }
}

#[derive(Debug)]
pub struct StorageError(String);

impl StorageError {
    pub fn new(title: impl Display) -> Self {
        Self(title.to_string())
    }
}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl ApiError for StorageError {
    fn error_type(&self) -> String {
        String::from("storage_error")
    }

    fn code(&self) -> ErrorCode {
        ErrorCode::StorageError
    }
}

#[derive(Debug)]
pub struct CoreApiError(String);

impl CoreApiError {
    pub fn new(title: impl Display) -> Self {
        Self(title.to_string())
    }
}

impl Display for CoreApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl ApiError for CoreApiError {
    fn error_type(&self) -> String {
        String::from("core_api_error")
    }

    fn code(&self) -> ErrorCode {
        ErrorCode::CoreApiError
    }
}

impl ApiError for SubmitError {
    fn error_type(&self) -> String {
        String::from("submit_error")
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::AccountCloseDisabled => ErrorCode::AccountCloseDisabled,
            Self::InvalidParams(_) => ErrorCode::InvalidParams,
            Self::UnsupportedFastProcessing => ErrorCode::UnsupportedFastProcessing,
            Self::IncorrectTx(_) => ErrorCode::IncorrectTx,
            Self::TxAdd(_) => ErrorCode::TxAddError,
            Self::InappropriateFeeToken => ErrorCode::InappropriateFeeToken,
            Self::CommunicationCoreServer(_) => ErrorCode::CommunicationCoreServer,
            Self::Internal(_) => ErrorCode::InternalError,
            Self::Other(_) => ErrorCode::Other,
        }
    }
}

impl ApiError for PriceError {
    fn error_type(&self) -> String {
        String::from("token_error")
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::TokenNotFound(_) => ErrorCode::TokenNotFound,
            Self::ApiError(_) => ErrorCode::ExternalApiError,
            Self::DBError(_) => ErrorCode::StorageError,
        }
    }
}
