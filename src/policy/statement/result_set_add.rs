use serde_derive::Deserialize;

use super::parameter::LocalParameter;

#[derive(Deserialize, Debug)]
pub struct ResultSetAdd {
    value: LocalParameter,
}
