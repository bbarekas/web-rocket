use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize, Debug)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}
