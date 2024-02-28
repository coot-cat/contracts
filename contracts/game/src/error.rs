use cosmwasm_std::{StdError, VerificationError};
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("{0}")]
    VerificationError(#[from] VerificationError),

    #[error("NoAnyCatNft")]
    NoAnyCatNft,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("UnrecognizedReplyId: {0}")]
    UnrecognizedReplyId(u64),

    #[error("Not enough funds to mint")]
    NotEnoughFundsToMint,
}
