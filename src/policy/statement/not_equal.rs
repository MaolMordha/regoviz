use serde_derive::Deserialize;

use super::parameter::OperandParameter;

#[derive(Deserialize, Debug)]
pub struct NotEqual {
    a: OperandParameter,
    b: OperandParameter,
}
