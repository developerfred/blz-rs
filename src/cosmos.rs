use std::{str::bytes, fmt, error};

use serde_json::{self, json};

const TX_COMMAND: &str = "/txs";
const TOKEN_NAME: &str = "ubnt";
const BROADCAST_MAX_RETRIES: i32 = 10;
// const BROADCAST_RETRY_INTERVAL time.Duration = time.Second


pub struct KeyValue {
	Key: String,
	Value: String,
}

pub struct KeyLease {
	Key: String,
	Lease: String,
} 

pub struct GasInfo {
	MaxGas: i32,
	MaxFee: i32,
	GasPrice: i32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct TransactionFeeAmount {
	pub Amount: String,
	pub Denom: String,
}

pub struct TransactionFee {
	pub Amount: Option<Vec<TransactionFeeAmount>>:
	Gas: String,
}


/// Error
#[derive(Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: i64,
    message: String,
    /// Optional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ParamsValue>,
}

impl ErrorResponse {
    /// Get error message
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Get error code
    pub fn code(&self) -> i64 {
        self.code
    }
}

impl fmt::Debug for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", json!(self))
    }
}
