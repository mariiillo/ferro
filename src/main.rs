mod entities;
mod use_cases;

fn main() {
    let todoList = use_cases::createTodoList(String::from("Einkaufswagen"));
}
