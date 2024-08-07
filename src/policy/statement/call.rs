use serde_derive::Deserialize;

use super::parameter::{LocalParameter, OperandParameter, Parameter, StringParameter};

#[derive(Deserialize, Debug)]
pub struct Call {
    #[serde(rename = "func")]
    function_name: StringParameter,

    #[serde(rename = "args")]
    arguments: Vec<OperandParameter>,

    result: LocalParameter,
}
