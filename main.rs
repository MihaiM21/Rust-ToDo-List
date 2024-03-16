use std::io;
use std::io::Write;

struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }
    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }
    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        } else {
            println!("Task not found!");
        }
    }
    fn print_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{} [{}] {}", index, if task.completed { "X" } else { " " }, task.description);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        match input {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                let description = description.trim().to_string();
                todo_list.add_task(description);
            }
            "2" => {
                println!("Enter task index to complete:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Failed to read input");
                let index = index_str.trim().parse::<usize>().expect("Invalid index");
                todo_list.complete_task(index);
            }
            "3" => {
                println!("Todo List:");
                todo_list.print_tasks();
            }
            "q" => {
                println!("Quitting...");
                break;
            }
            _ => println!("Invalid input"),
        }
    }
}

fn print_menu() {
    println!("\nToDo List Program");
    println!("1. Add a new task");
    println!("2. Complete task");
    println!("3. Print tasks");
    println!("q. Quit program");
    print!("> ");
    io::stdout().flush().unwrap();
}