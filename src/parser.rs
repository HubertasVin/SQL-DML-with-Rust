use crate::statement_adt::ParsedStatement;

pub fn parse_to_adt(statement: &str) -> Result<ParsedStatement, String> {
    let parts: Vec<&str> = statement.trim().split_whitespace().collect();
    
    match parts.as_slice() {
        ["show", "tables"] => Ok(ParsedStatement::ShowTables),
        ["show", "table", table_name] => Ok(ParsedStatement::ShowTable(table_name.to_string())),
        // You can add more patterns here
        _ => Err("Unrecognized statement".to_string()),
    }
}