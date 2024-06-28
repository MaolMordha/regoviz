mod assign_var;
mod assign_var_once;
mod call;
mod dot;
mod is_defined;
mod make_object;
mod not_equal;
mod object_insert;
pub mod parameter;
mod reset_local;
mod result_set_add;
mod return_local;

use assign_var::AssignVar;
use assign_var_once::AssignVarOnce;
use call::Call;
use dot::Dot;
use is_defined::IsDefined;
use make_object::MakeObject;
use not_equal::NotEqual;
use object_insert::ObjectInsertStatement;
use reset_local::ResetLocal;
use result_set_add::ResultSetAdd;
use return_local::ReturnLocal;
use serde_derive::Deserialize;

// TODO find better names

#[derive(Deserialize, Debug)]
pub struct SourceFileMetadata {
    file: usize,
    col: usize,
    row: usize,
}

#[derive(Deserialize, Debug)]
pub struct Statement<T> {
    #[serde(flatten)]
    specific_statement: T,

    #[serde(flatten)]
    source_file_metadata: SourceFileMetadata,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "stmt")]
pub enum StatementEnum {
    #[serde(rename = "AssignVarOnceStmt")]
    AssignVarOnce(Statement<AssignVarOnce>),
    #[serde(rename = "AssignVarStmt")]
    AssignVar(Statement<AssignVar>),
    #[serde(rename = "CallStmt")]
    Call(Statement<Call>),
    #[serde(rename = "DotStmt")]
    Dot(Statement<Dot>),
    #[serde(rename = "IsDefinedStmt")]
    IsDefined(Statement<IsDefined>),
    #[serde(rename = "MakeObjectStmt")]
    MakeObject(Statement<MakeObject>),
    #[serde(rename = "NotEqualStmt")]
    NotEqual(Statement<NotEqual>),
    #[serde(rename = "ObjectInsertStmt")]
    ObjectInsert(Statement<ObjectInsertStatement>),
    #[serde(rename = "ResetLocalStmt")]
    ResetLocal(Statement<ResetLocal>),
    #[serde(rename = "ResultSetAddStmt")]
    ResultSetAdd(Statement<ResultSetAdd>),
    #[serde(rename = "ReturnLocalStmt")]
    ReturnLocal(Statement<ReturnLocal>),
}
