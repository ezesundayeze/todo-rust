mod create_todo;
use std::io;
mod init;
use rusqlite::{ Connection};

fn main(){
    init::init_db();
    
    let mut title = String::new();
    let mut description = String::new();
    let mut due_date = String::new();
    let conn = Connection::open("todo.db");

io::stdin()
    .read_line(&mut title)
    .expect("failed to read line");

io::stdin()
    .read_line(&mut description)
    .expect("failed to read line");

io::stdin()
    .read_line(&mut due_date)
    .expect("failed to read line");

    create_todo::create(title, description, due_date, conn);
    println!("Hello, world!");
}
