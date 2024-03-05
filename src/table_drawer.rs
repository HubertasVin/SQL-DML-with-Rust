use crate::data_frame::DataFrame;
pub fn draw_table(data_frame: DataFrame) {
    let column_count = data_frame.columns.len();
    let termsize::Size { rows: _, cols } = termsize::get().unwrap();
    let mut column_size = (cols as usize - (column_count + 1)) / column_count;
    if column_size < 10 {
        column_size = 10;
    }

    println!("+{}+", "=".repeat(column_size * column_count + column_count - 1));
    print!("|");
    for column in data_frame.columns {
        print!("{:<width$}|", column.name, width = column_size);
    }
    println!();
    println!("+{}+", "=".repeat(column_size * column_count + column_count - 1));
    for (i, row) in data_frame.rows.iter().enumerate().clone() {
        print!("|");
        for value in row.clone() {
            print!("{:<width$}|", value.to_string(), width = column_size);
        }
        println!();

        if i < data_frame.rows.len() - 1 {
            println!("+{}+", "-".repeat(column_size * column_count + column_count - 1));
        }
    }
    println!("+{}+", "=".repeat(column_size * column_count + column_count - 1));
}
