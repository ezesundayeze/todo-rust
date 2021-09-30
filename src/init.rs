use rusqlite::{Connection};

pub fn init_db() {
    #[derive(Debug)]
    pub struct Todo {
        id: i32,
        title: String,
        description: Option<Vec<u8>>,
        status: String,
        due_date: Option<String>,
    }
    
    let conn = Connection::open_in_memory().expect("DB not created");
    conn.execute(
        "CREATE TABLE todo (
                  id              INTEGER PRIMARY KEY,
                  title            TEXT NOT NULL,
                  description            TEXT,
                  status            TEXT NOT NULL DEFAULT 'inactive',
                  due_date            TEXT
                  )",
        [],
    ).expect("Table creation failed");

}
