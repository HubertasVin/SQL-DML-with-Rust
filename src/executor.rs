use crate::{
    data_frame::{Column, ColumnType, DataFrame, Value}, database::{self, database}, statement_adt::ParsedStatement
};

pub fn execute_statement(statement: ParsedStatement) -> Result<DataFrame, &'static str> {
    match statement {
        ParsedStatement::ShowTables => show_tables(),
        ParsedStatement::ShowTable(name) => {
            show_table(&name)
        }
        _ => Err("Unable to execute statement."),
    }
}

fn show_tables() -> Result<DataFrame, &'static str> {
    let vec_of_tables = database();
    if vec_of_tables.is_empty() {
        return Err("No tables found.");
    }
    let vec_of_table_names: Vec<&str> = vec_of_tables.iter().map(|(s, _)| *s).collect();
    let dataframe_of_table_names = DataFrame {
        columns: vec![Column {
            name: "tables".to_string(),
            column_type: ColumnType::StringType,
        }],
        rows: vec_of_table_names.iter().map(|s| vec![Value::StringValue(s.to_string())]).collect(),
    };
    Ok(dataframe_of_table_names)
}

fn show_table(name: &str) -> Result<DataFrame, &'static str> {
    let table_result = database::get_table(name);
    match table_result {
        Ok((_, dataframe)) => return Ok(dataframe),
        Err(err) => return Err(err),
        
    }
}
