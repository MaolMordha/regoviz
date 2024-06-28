use serde_derive::Deserialize;

use super::parameter::{LocalParameter, OperandParameter};

#[derive(Deserialize, Debug)]
pub struct AssignVarOnce {
    source: OperandParameter,
    target: LocalParameter,
}
