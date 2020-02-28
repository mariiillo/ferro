use crate::entities;

pub fn createTodoList(name: String) -> entities::TodoList {
    let mut todolist = entities::TodoList { todos: Vec::new() };
    todolist.setName(name);
    todolist;
}