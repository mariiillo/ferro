use crate::entities;

pub fn create_todo_list(name: String) -> entities::TodoList {
    entities::TodoList { todos: Vec::new(), name }
}