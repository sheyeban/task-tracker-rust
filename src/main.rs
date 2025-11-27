use std::io;
use std::io::Write;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("ошибочка!");

    s.trim().to_string()
}
    
fn choose_status() -> TaskStatus {
    loop {
        println!("выбери статус таски ");
        println!("1. не сделано");
        println!("2. в процессе");
        println!("3. готово");
        
        let choice = input("твой выбор: ");

        match choice.trim() {
            "1" => return TaskStatus::Unready,
            "2" => return TaskStatus::InProcess,
            "3" => return TaskStatus::Done,
            _ => return TaskStatus::Undefined
        }
    }
}
    
fn choose_priority() -> TaskPriority {
    loop {
        println!("выбери приоритет таски ");
        println!("1. низкий");
        println!("2. средний");
        println!("3. высокий");
        
        let choice = input("твой выбор: ");

        match choice.trim() {
            "1" => return TaskPriority::Low,
            "2" => return TaskPriority::Medium,
            "3" => return TaskPriority::High,
            _ => return TaskPriority::Undefined
        }
    }
}
    
fn show_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("тут ничего нет:(");
        return;
    }
    println!("все таски");

    for task in tasks {
        task.print();
    }
}

fn show_tasks_id(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("тут ничего нет:(");
        return;
    }

    println!("\n---------------список---------------");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {}", i, task.name);
    }
    println!(); 
}

fn select_task(tasks: &Vec<Task>) -> Option<usize> {
    show_tasks_id(tasks);

    let id_str = input("введи номер таски ");
    let id: usize = match id_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("попробуй еще раз");
            return None;
        }
    };

    if id >= tasks.len() {
        println!("такого таска нет:(");
        None
    } else {
        Some(id)
    }
}

fn update_status(tasks: &mut Vec<Task>) {
    if let Some(i) = select_task(tasks) {
        tasks[i].change_status();
    }
}
fn update_priority(tasks: &mut Vec<Task>) {
    if let Some(i) = select_task(tasks) {
        tasks[i].change_priority();
    }
} 

fn update_description(tasks: &mut Vec<Task>) {
    if let Some(i) = select_task(tasks) {
        tasks[i].change_description();
    }
} 

fn update_deadline(tasks: &mut Vec<Task>) {
    if let Some(i) = select_task(tasks) {
        tasks[i].change_deadline();
    }
} 

fn update_name(tasks: &mut Vec<Task>) {
    if let Some(i) = select_task(tasks) {
        tasks[i].change_name();
    }
}

fn save_file(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks)
        .expect("ошибочка");

    std::fs::write("tasks.json", json)
        .expect("ошибочка");

    println!("все сохранилось!!");
}
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, serde::Serialize)]

enum TaskStatus {
    Unready,
    InProcess,
    Done,
    Undefined
}

#[derive(Debug, serde::Serialize)]
enum TaskPriority {
    Low,
    Medium,
    High,
    Undefined
}

#[derive(Debug, serde::Serialize)]
struct User {
    id: u32,
    name: String
}

#[derive(Debug, serde::Serialize)]
struct Task {
    id: u32,
    name: String,
    deadline: String,
    description: String,
    status: TaskStatus,
    priority: TaskPriority,
    creator_id: u32,
    executor_id: u32
}

impl TaskStatus {
    fn to_ru(&self) -> &str {
        match self {
            TaskStatus::Unready => "не сделано",
            TaskStatus::InProcess => "в процессе",
            TaskStatus::Done => "готово",
            TaskStatus::Undefined => "неизвестно"
            
        }
    }
    
}

impl TaskPriority {
    fn to_ru(&self) -> &str {
        match self {
            TaskPriority::Low => "низкий",
            TaskPriority::Medium => "средний",
            TaskPriority::High => "высокий",
            TaskPriority::Undefined => "неизвестно"
            
        }
    }
    
}

impl Task {
    fn new_task(
        id: u32,
        creator_id: u32,
        executor_id: u32
    ) -> Task {
        let name = input("название таски ");
        let deadline = input("дедлайн ");
        let description = input("описание таски ");
        let status = choose_status();
        let priority = choose_priority();

        Task {
            id,
            name,
            deadline,
            description,
            status,
            priority,
            creator_id,
            executor_id,
        }
    }

    fn print(&self) {
        println!("----------------------------------");
        println!("ID: {}", self.id);
        println!("название: {}", self.name);
        println!("дедлайн: {}", self.deadline);
        println!("описание: {}", self.description);
        println!("статус: {:?}", self.status.to_ru());
        println!("приоритет: {:?}", self.priority.to_ru());
        println!("создатель: {}", self.creator_id);
        println!("исполнитель: {}", self.executor_id);
        println!("----------------------------------\n");
}

    
    fn change_status(&mut self) {
        println!("меняем статус таски {}", self.name);

        let new_status = choose_status();

        self.status = new_status;

        println!("статус обновлен");
    }

    fn change_priority(&mut self) {
        println!("меняем приоритет таски {}", self.name);

        let new_priority = choose_priority();

        self.priority = new_priority;

        println!("приоритет обновлен");
    }
    
    fn change_description(&mut self) {
        println!("меняем описание таски {}", self.name);

        let new_description = input("введи новое описание ");

        self.description = new_description;

        println!("описание обновлено");
    }

    fn change_deadline(&mut self) {
        println!("меняем дедлайн таски {}", self.name);

        let new_deadline = input("введи новый дедлайн ");

        self.deadline = new_deadline;

        println!("дедлайн обновлен");
    }

    fn change_name(&mut self) {
        println!("меняем название таски {}", self.name);

        let new_name = input("введи новое название ");

        self.name = new_name;

        println!("название обновлено")
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("---------------меню---------------");
        println!("1. создать таску");
        println!("2. показать таски");
        println!("3. изменить название таски");
        println!("4. изменить описание таски");
        println!("5. изменить статус таски");
        println!("6. изменить приоритет таски");
        println!("7. изменить дедлайн таски");
        println!("8. выход");
        println!("----------------------------------\n");
        
        let choice = input("выбери действие ");

        match choice.trim() {
            "1" => {
                let task = Task::new_task(tasks.len() as u32 + 1, 1, 1);
                tasks.push(task);
            }
            "2" => show_tasks(&tasks),
            "3" => update_name(&mut tasks),
            "4" => update_description(&mut tasks),
            "5" => update_status(&mut tasks), 
            "6" => update_priority(&mut tasks),
            "7" => update_deadline(&mut tasks),
            "8" => {
                println!("сохраняемся...");
                save_file(&tasks);
                println!("выход...");
                break;
            }
            _ => println!("тут ничего нет:("),  
        }
    }
}

