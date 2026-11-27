use wincode::{SchemaRead, SchemaWrite};

#[derive(SchemaRead, SchemaWrite, Debug)]
pub enum Commands {
    Create,
    Join,
}

#[derive(SchemaRead, SchemaWrite, Debug)]
pub struct Message {
    pub username: String,
    pub roomname: String,
    pub command: Commands,
    pub userid: String,
}
