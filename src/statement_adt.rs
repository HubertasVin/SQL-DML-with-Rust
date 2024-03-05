#![allow(dead_code)]
use crate::data_frame::Value;

type TableName = String;

#[derive(Debug)]
pub enum ParsedStatement {
    ShowTables,
    ShowTable(TableName),
    Select(TableName, Vec<String>),
    // Select(Vec<String>, Vec<TableName>, Option<Vec<Operator>>, Option<String>),
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