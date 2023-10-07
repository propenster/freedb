use std::collections::HashMap;

use freedb::{Column, ColumnData, DataType, Database, RowData, Table};
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let mut db = Database::default();
    db.name = String::from("MyDatabase");

    let mut columns: Vec<Column> = Vec::new();
    let column_id = Column::new("Id", DataType::from("string"));
    let column_name = Column::new("Name", DataType::from("string"));
    columns.push(column_id);
    columns.push(column_name);

    let mut table = Table::new("MyTable", &columns);

    //how to represent data in tables -> columns -> and db
    let query: &str = "INSERT INTO MyDatabase.MyTable (Id, Name) VALUES ('1', 'Ben Frank')";
    let mut row_data: Vec<RowData> = Vec::new();
    //let row1 = RowData::new()
    //let cols = table.clone().columns;
    // let mut col_data: Vec<ColumnData> = vec![];
    // for c in cols {
    //     let cr = c.clone();
    //     let value: String =
    //         get_value_from_query(query, &cr.name, &cr.data_type).unwrap_or("".into());
    //     let c_data = ColumnData::new(cr.name, value, cr.data_type);
    //     col_data.push(c_data);
    // }

    //Option 2 for extracting column - value pairs...
    if let Some(col_data) = extract_column_values_from_insert(query, &columns) {
        //push all columns into a row...
        let row = RowData::new(col_data);

        //push row into rows... we may have more than 1
        row_data.push(row);
        //push row_data -> array Of Rows into the table(Cupboard)
        table.row_data = row_data;
    }

    db.tables.push(table.clone());

    println!("Database => {:?}", db);
}

fn get_value_from_query(query: &str, name: &String, data_type: &DataType) -> Option<String> {
    // Construct a regular expression pattern to match the column value
    let pattern = format!("{}\\s*=\\s*'([^']*)'", name);
    let re = Regex::new(&pattern).expect("Invalid regex pattern");

    // Search for the pattern in the query string
    if let Some(captures) = re.captures(query) {
        if let Some(value) = captures.get(1) {
            return Some(value.as_str().to_string());
        }
    }

    None
}

fn extract_column_values_from_insert(query: &str, cols: &Vec<Column>) -> Option<Vec<ColumnData>> {
    // Define a regular expression pattern to capture column names and their values
    let pattern = r"INSERT INTO \w+\.\w+ \((?P<columns>[^)]+)\) VALUES \((?P<values>[^)]+)\)";
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Search for the pattern in the query string
    if let Some(captures) = re.captures(query) {
        if let (Some(column_match), Some(value_match)) =
            (captures.name("columns"), captures.name("values"))
        {
            // Split column names and values into separate parts
            let columns = column_match.as_str().split(", ");
            let values = value_match.as_str().split(", ");

            // Create a HashMap to store column names and their values
            //let mut result = HashMap::new();
            let mut col_data: Vec<ColumnData> = Vec::new();

            for (column, value) in columns.zip(values) {
                let t = cols
                    .iter()
                    .find(|f| f.name.to_lowercase() == column.to_lowercase())
                    .clone()
                    .unwrap();
                let c_data = ColumnData::new(
                    column.trim().to_string(),
                    value.trim().to_string(),
                    t.clone().data_type,
                );
                col_data.push(c_data);
                //result.insert(column.trim().to_string(), value.trim().to_string());
            }

            return Some(col_data);
        }
    }

    None
}
