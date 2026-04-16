use aws_sdk_lambda::{
    error::SdkError,
    operation::{add_permission::AddPermissionError, create_function::CreateFunctionError},
};
use aws_sdk_s3::operation::{
    create_bucket::CreateBucketError,
    put_bucket_notification_configuration::PutBucketNotificationConfigurationError,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::fmt::{Display, Formatter};
use syn::Error;

#[derive(Debug)]
pub enum IacError {
    BucketError(String),
    LambdaError(String),
    EventError(String),
}

impl IacError {
    pub fn into_compile_error(self) -> TokenStream {
        match self {
            IacError::BucketError(message) => Error::new(
                Span::call_site(),
                format!("bucket could not be created: {}", message),
            )
            .into_compile_error()
            .into(),
            IacError::LambdaError(message) => Error::new(
                Span::call_site(),
                format!("lambda could not be created: {}", message),
            )
            .into_compile_error()
            .into(),
            IacError::EventError(message) => Error::new(
                Span::call_site(),
                format!(
                    "event to link bucket and lambda could not be created: {}",
                    message
                ),
            )
            .into_compile_error()
            .into(),
        }
    }
}

impl std::error::Error for IacError {}

impl Display for IacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("retrieval error")
    }
}

macro_rules! generate_from_error {
    ($mine:expr, $aws:ty) => {
        impl From<SdkError<$aws>> for IacError {
            fn from(value: SdkError<$aws>) -> Self {
                let message = format!("{:?}", value);
                $mine(message)
            }
        }
    };
}

generate_from_error!(IacError::BucketError, CreateBucketError);
generate_from_error!(IacError::LambdaError, CreateFunctionError);
generate_from_error!(
    IacError::EventError,
    PutBucketNotificationConfigurationError
);
generate_from_error!(IacError::EventError, AddPermissionError);
