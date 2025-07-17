use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum ErrorCode {
    #[msg("This pool is locked")]
    PoolLocked,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Invalid precision")]
    InvalidPrecision,

    #[msg("Overflow")]
    Overflow,

    #[msg("Underflow")]
    Underflow,

    #[msg("Invalid Fee Amount")]
    InvalidFeeAmount,

    #[msg("Insufficient Balance")]
    InsufficientBalance,

    #[msg("Zero Balance")]
    ZeroBalance,

}
 
impl From<CurveError> for ErrorCode {
    fn from(error: CurveError) -> ErrorCode {
        match error {
            CurveError::InvalidPrecision => ErrorCode::InvalidPrecision,
            CurveError::Overflow => ErrorCode::Overflow,
            CurveError::Underflow => ErrorCode::Underflow,
            CurveError::InvalidFeeAmount => ErrorCode::InvalidFeeAmount,
            CurveError::InsufficientBalance => ErrorCode::InsufficientBalance,
            CurveError::ZeroBalance => ErrorCode::ZeroBalance,
            CurveError::SlippageLimitExceeded => ErrorCode::SlippageExceeded,
        }
    } 
}