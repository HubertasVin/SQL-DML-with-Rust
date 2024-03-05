use crate::data_frame::{Column, ColumnType, DataFrame, Value};

pub fn table_employees() -> (&'static str, DataFrame) {
    (
        "employees",
        DataFrame {
            columns: vec![
                Column {
                    name: "id".to_string(),
                    column_type: ColumnType::IntegerType,
                },
                Column {
                    name: "name".to_string(),
                    column_type: ColumnType::StringType,
                },
                Column {
                    name: "surname".to_string(),
                    column_type: ColumnType::StringType,
                },
            ],
            rows: vec![
                vec![
                    Value::IntegerValue(1),
                    Value::StringValue("Vi".to_string()),
                    Value::StringValue("Po".to_string()),
                ],
                vec![
                    Value::IntegerValue(2),
                    Value::StringValue("Ed".to_string()),
                    Value::StringValue("Dl".to_string()),
                ],
                vec![
                    Value::IntegerValue(3),
                    Value::StringValue("Hu".to_string()),
                    Value::StringValue("Vi".to_string()),
                ],
                vec![
                    Value::IntegerValue(4),
                    Value::StringValue("Pa".to_string()),
                    Value::StringValue("Dl".to_string()),
                ],
            ],
        },
    )
}

pub fn table_employees_salary() -> (&'static str, DataFrame) {
    (
        "employeesSalary",
        DataFrame {
            columns: vec![
                Column {
                    name: "id".to_string(),
                    column_type: ColumnType::IntegerType,
                },
                Column {
                    name: "salary".to_string(),
                    column_type: ColumnType::StringType,
                },
            ],
            rows: vec![
                vec![
                    Value::IntegerValue(1),
                    Value::StringValue("900".to_string()),
                ],
                vec![
                    Value::IntegerValue(2),
                    Value::StringValue("300".to_string()),
                ],
                vec![
                    Value::IntegerValue(3),
                    Value::StringValue("400".to_string()),
                ],
                vec![
                    Value::IntegerValue(4),
                    Value::StringValue("1000".to_string()),
                ],
            ],
        },
    )
}

pub fn database() -> Vec<(&'static str, DataFrame)> {
    vec![table_employees(), table_employees_salary()]
}

pub fn get_table(name: &str) -> Result<(&str, DataFrame), &'static str> {
    let result = database().into_iter().find(|table| table.0 == name);
    match result {
        Some(table) => Ok(table),
        None => Err("Table not found"),
    }
}