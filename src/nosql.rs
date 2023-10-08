

use serde_json::Value;

//row... document... item of a Table/Collection
#[derive(Debug,Default, Clone )]
pub struct Document{
    pub data: Value,
}