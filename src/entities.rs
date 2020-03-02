pub struct TodoList {
    pub todos: Vec<Todo>,
    pub name: String,
}

pub struct Todo {
    description: String,
    completed: bool,
}