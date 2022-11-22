use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("ERR_STD|{0}")]
    Std(#[from] StdError),

    #[error("ERR_NO_SCHOLARSHIP|Sender not in scholarship list and must pay")]
    Unauthorized {},

    #[error("ERR_UNKNOWN_REPLY|Unknown reply ID: {id:?}")]
    UnknownReplyID { id: u64 },

    #[error("ERR_REPLY_ERROR|{code:?}|{msg:?}")]
    ReplyError { code: u64, msg: String },

    #[error("{code:?}|{msg:?}")]
    CustomError { code: String, msg: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror for details.
}
