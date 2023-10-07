use core::panic;
use std::default;

#[derive(Debug, Clone)]
pub enum DataType {
    String(u64),
    Number,
    Decimal,
    Boolean,
}
impl From<String> for DataType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "string" => Self::String(255),
            "number" => Self::Number,
            "decimal" => Self::Decimal,
            "boolean" => Self::Boolean,
            _ => unimplemented!(),
        }
    }
}
impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "string" => Self::String(255),
            "number" => Self::Number,
            "decimal" => Self::Decimal,
            "boolean" => Self::Boolean,
            _ => unimplemented!(),
        }
    }
}
impl Default for DataType {
    fn default() -> Self {
        // Choose a default variant here, e.g., DataType::Int
        DataType::String(255)
    }
}



#[derive(Debug,  Clone)]
pub struct ColumnData{
    pub name: String,
    pub value: String,
    pub data_type: DataType,
}
impl ColumnData{
    pub fn new(name: impl Into<String>, value: impl Into<String>, data_type: DataType) -> Self{
        Self { name: name.into(), value: value.into(), data_type: data_type }
    }
}

#[derive(Debug,Default, Clone)]
pub struct Column {

    pub name: String,
    //data_type: String,
    pub data_type: DataType,
    pub value: String, //an intermediate of String...
}

impl Column {
    // fn default() -> Self {
    //     Self {
    //         name: "".into(),
    //         data_type: "string".to_owned(),
    //         value: "".into(),
    //     }
    // }
    pub fn new(name: impl Into<String> , data_type: DataType) -> Self{
        Self { name: name.into(), data_type: data_type, value: String::new() }
    }
}


#[derive(Debug, Default, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub row_data: Vec<RowData> //this is ROWS in this shitable
}

impl Table {
    fn default() -> Self{
        Self { name: "".into(), columns: vec![], row_data: vec![] }
    }
    pub fn new(name: impl Into<String>, columns: &Vec<Column>) -> Self{
        Self { name: name.into(), columns: columns.to_owned(), row_data: vec![] }
    }
    pub fn insert_one(&mut self, one_row: &RowData){
        self.row_data.push(one_row.clone())
    }
    pub fn insert_many(&mut self, many_rows: &Vec<RowData>){
        for r in many_rows{
            self.row_data.push(r.clone())
        }
    }
    
}
#[derive(Debug,Default, Clone)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}

impl Database{
    fn default() -> Self{
        Self { name: "".into(), tables: vec![] }
    }
}

// #[derive(Debug, Clone )]
// pub struct RowData<'a>{
//     pub table: &'a Table,
//     pub columns: Vec<Column>,
// }
// impl<'a> RowData<'a>{
//     // create Db, create Table with columns...
//     //insert into Table (Id, Name) VALUES ('1', 'Faith') => create RowData...to Table... 
//     //does Table Tbl need to have Vec<RowData>?
//     pub fn new(table: &'a Table, column_data: Vec<Column>) -> Self{
//         Self { table: table, columns: column_data }
//     }
// }

#[derive(Debug,Default, Clone )]
pub struct RowData{
    pub columns: Vec<ColumnData>,
}
impl RowData{
    // create Db, create Table with columns...
    //insert into Table (Id, Name) VALUES ('1', 'Faith') => create RowData...to Table... 
    //does Table Tbl need to have Vec<RowData>?
    pub fn new(column_data: Vec<ColumnData>) -> Self{
        Self { columns: column_data }
    }
}

