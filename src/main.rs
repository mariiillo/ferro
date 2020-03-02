mod entities;
mod use_cases;

fn main() {
    let todo_list = use_cases::create_todo_list(String::from("Einkaufswagen"));
}