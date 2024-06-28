use serde_derive::Deserialize;

use super::parameter::LocalOperandParameter;

#[derive(Deserialize, Debug)]
pub struct IsDefined {
    source: LocalOperandParameter,
}
