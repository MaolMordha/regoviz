use serde_derive::Deserialize;

use super::parameter::{LocalParameter, OperandParameter};

#[derive(Deserialize, Debug)]
pub struct ObjectInsertStatement {
    key: OperandParameter,
    value: OperandParameter,
    object: LocalParameter,
}
