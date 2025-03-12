use thiserror::Error;

#[derive(Error, Debug)]
pub enum DexScreenerError {
    #[error("Error fetching or processing data from DexScreener API")]
    DataError,
    
    #[error("Incorrect baseToken provided")]
    TokenError,
}
