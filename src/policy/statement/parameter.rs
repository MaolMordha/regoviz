use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Parameter {
    #[serde(rename = "local")]
    Local(LocalParameter),
    #[serde(rename = "int32")]
    Int32(Int32Parameter),
    #[serde(rename = "int64")]
    Int64(Int64Parameter),
    #[serde(rename = "uin32")]
    UInt32(UInt32Parameter),
    #[serde(rename = "string")]
    String(StringParameter),
    // TODO array
    #[serde(rename = "operand")]
    Operand(OperandParameter),
}

#[derive(Deserialize, Debug)]
pub struct LocalParameter(i32);

#[derive(Deserialize, Debug)]
pub struct Int32Parameter(i32);

#[derive(Deserialize, Debug)]
pub struct Int64Parameter(i64);

#[derive(Deserialize, Debug)]
pub struct UInt32Parameter(u32);

#[derive(Deserialize, Debug)]
pub struct StringParameter(String);

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum OperandParameter {
    #[serde(rename = "local")]
    Local(LocalOperandParameter),
    #[serde(rename = "bool")]
    Bool(BoolOperandParameter),
    #[serde(rename = "string_index")]
    StringConstant(StringConstantOperandParameter),
}

#[derive(Deserialize, Debug)]
pub struct LocalOperandParameter(i32);

#[derive(Deserialize, Debug)]
pub struct BoolOperandParameter(bool);

#[derive(Deserialize, Debug)]
pub struct StringConstantOperandParameter(i64);

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::{LocalParameter, Parameter};

    #[test]
    pub fn deserialize_local_from_number_json_token() {
        let json = "1";
        let local: LocalParameter = serde_json::from_str(json).unwrap();

        assert_eq!(local.0, 1);
    }

    #[test]
    pub fn deserialize_local_from_type_tagged_json_object() {
        let json = r#"
            {
                "type": "local",
                "value": 1
            }
            "#;

        let parameter: Parameter = serde_json::from_str(json).unwrap();

        assert_matches!(parameter, Parameter::Local(p) if p.0 == 1);
    }
}
