use std::fmt;

#[derive(Debug)]
pub struct FailedProofCreateError;


impl fmt::Display for FailedProofCreateError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "Failed to create proof")
    }
}

impl std::error::Error for FailedProofCreateError {}

#[derive(Debug)]
pub struct QuotaLimitReachError;

impl fmt::Display for QuotaLimitReachError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quota limit reached")
    }
}

impl std::error::Error for QuotaLimitReachError {}

#[derive(Debug)]
pub struct NotAuthorized;

impl fmt::Display for NotAuthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not authorized")
    }
}

impl std::error::Error for NotAuthorized {}