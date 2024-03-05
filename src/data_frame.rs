#![allow(dead_code)]
pub enum ColumnType {
    IntegerType,
    StringType,
    BoolType,
}

pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    IntegerValue(i64),
    StringValue(String),
    BoolValue(bool),
    NullValue,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::IntegerValue(i) => i.to_string(),
            Value::StringValue(s) => s.to_string(),
            Value::BoolValue(b) => b.to_string(),
            Value::NullValue => "NULL".to_string(),
        }
    }
}

type Row = Vec<Value>;

pub struct DataFrame {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}