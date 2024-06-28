use serde_derive::Deserialize;

use super::parameter::{LocalParameter, OperandParameter};

#[derive(Deserialize, Debug)]
pub struct AssignVar {
    source: OperandParameter,
    target: LocalParameter,
}
