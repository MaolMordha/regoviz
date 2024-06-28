use serde_derive::Deserialize;

use super::statement::StatementEnum;

#[derive(Deserialize, Debug)]
pub struct Block {
    #[serde(rename = "stmts")]
    statements: Vec<StatementEnum>,
}
