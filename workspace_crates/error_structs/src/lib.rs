use std::fmt::Debug;

use tracing::error;


#[derive(Debug)]
pub struct PublicError {
    pub message: String,
}
impl PublicError {
    pub fn new<E: Debug>(s: E) -> Self {
        let message = format!("{s:?}");
        error!("InternalAppError: {}", message.clone());
        Self { message }
    }
}

#[derive(Debug)]
pub struct InternalAppError {
    pub message: String,
}
impl InternalAppError {
    pub fn new<E: Debug>(s: E) -> Self {
        let message = format!("{s:?}");
        error!("InternalAppError: {}", message.clone());
        Self { message }
    }
}
#[derive(Debug)]
pub struct ExpectedToGetButFailed {
    pub message: String,
}
impl ExpectedToGetButFailed {
    pub fn new<E: Debug>(s: E) -> Self {
        let message = format!("{s:?}");
        error!(
            "Expected the thingie to exist but apparently it does not: {}",
            message.clone()
        );
        Self { message }
    }
}

#[derive(Debug)]
pub struct SomeSortOfError {
    pub message: String,
}
impl SomeSortOfError {
    pub fn new<E: Debug>(s: E) -> Self {
        let message = format!("{s:?}");
        println!("SomeSortOfError: {}", message.clone());
        Self { message }
    }
}




#[derive(Debug)]
pub struct StreamResponseWriterError;