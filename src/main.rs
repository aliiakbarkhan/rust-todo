fn main() {
    let mut todos: Vec<String> = Vec::new();

    todos.push(String::from("Learn Rust"));
    todos.push(String::from("Build a project"));
    todos.push(String::from("Push to Github"));

    println!("my tools");
    for (i, todo) in todos.iter().enumerate(){
        println!(" {}. {}", i+1, todo);
    }
}
