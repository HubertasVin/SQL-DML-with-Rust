#![allow(dead_code)]
use crate::data_frame::Value;
use std::fmt::{Debug, Display};

type TableName = String;
type Columns = Vec<String>;
#[derive(Debug)]
pub struct Clause {
    pub column: String,
    pub operator: String,
    pub value: Value,

}

#[derive(Debug)]
pub enum ParsedStatement {
    StringLiteral(String),
    NumberLiteral(Value),
    ShowTables,
    ShowTable(TableName),
    Select(TableName, Columns, Option<Vec<Clause>>),
    LoadDatabase,
    SaveDatabase,
    // Update(TableName, Vec<(String, Value)>, Option<Vec<Operator>>),
    Insert(TableName, Vec<(String, Value)>),
    // Delete(TableName, Option<Vec<Operator>>),
    // Now(Vec<String>, TableName, Option<Vec<Operator>>),
    LoadFileContent,
    // Where(Vec<Operator>),
    // The last ParsedStatement variant seems to be self-referential or incorrect,
    // so I'm omitting it; typically, you wouldn't include the type itself as a variant.
}

impl Display for ParsedStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}