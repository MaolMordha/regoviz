use serde_derive::Deserialize;

use super::parameter::{LocalParameter, OperandParameter};

#[derive(Deserialize, Debug)]
pub struct Dot {
    source: OperandParameter,
    key: OperandParameter,
    target: LocalParameter,
}
