use serde_derive::Deserialize;

use super::{block::Block, statement::parameter::LocalParameter};

#[derive(Deserialize, Debug)]
pub struct Function {
    name: String,
    r#return: LocalParameter,    // TODO not sure about LocalParameter
    params: Vec<LocalParameter>, // TODO not sure about LocalParameter
    path: Vec<String>,
    blocks: Vec<Block>,
}
