use rusqlite::{ Connection, params };
use rand::Rng;

pub fn create(title: String, description: String, due_date: String, conn: Connection) {
    let secret = rand::thread_rng().gen_range(10..50000);
    pub struct Todo {
        id: i32,
        title: String,
        description: String,
        status: String,
        due_date: String,
    }

    let todo = Todo {
        id: secret,
        title: title,
        description: description,
        status: "active".to_string(),
        due_date: due_date,
    };

    conn.execute(
        "INSERT INTO todo (id, title, description, due_date) VALUES (?1, ?2, ?3, ?4)",
        params![todo.id, todo.title, todo.description, todo.due_date],
    );
}