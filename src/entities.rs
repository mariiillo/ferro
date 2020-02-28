pub struct TodoList {
    todos: Vec<Todo>,
    name: String,
}

impl TodoList {
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}

struct Todo {
    description: String,
    completed: bool,
}