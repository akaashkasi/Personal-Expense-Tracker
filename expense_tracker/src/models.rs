use bcrypt::verify;
use rusqlite::{params, Connection, OptionalExtension, Result};

pub struct Expense {
    pub id: i32,
    pub date: String,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub payment_method: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String, //used password_hash instead of password for security reasons
}

pub fn create_expense_table() -> Result<()> {
    let conn = Connection::open("expenses.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            description TEXT,
            payment_method TEXT
            )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
            )",
        [],
    )?;
    Ok(())
}

pub fn add_expense(expense: &Expense) -> Result<()> {
    let conn = Connection::open("expenses.db")?;
    conn.execute(
        "INSERT INTO expenses (date, amount, category, description, payment_method) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![expense.date, expense.amount, expense.category, expense.description, expense.payment_method],
    )?;
    Ok(())
}

pub fn get_expenses() -> Result<Vec<Expense>> {
    let conn = Connection::open("expenses.db")?;
    let mut stmt = conn
        .prepare("SELECT id, date, amount, category, description, payment_method FROM expenses")?;
    let expense_iter = stmt.query_map([], |row| {
        Ok(Expense {
            id: row.get(0)?,
            date: row.get(1)?,
            amount: row.get(2)?,
            category: row.get(3)?,
            description: row.get(4)?,
            payment_method: row.get(5)?,
        })
    })?;

    let mut expenses = Vec::new();
    for expense in expense_iter {
        expenses.push(expense?);
    }
    Ok(expenses)
}

pub fn delete_expense(expense_id: i32) -> Result<()> {
    let conn = Connection::open("expenses.db")?;
    conn.execute("DELETE FROM expenses WHERE id = ?1", params![expense_id])?;
    Ok(())
}

#[derive(Debug)]
pub enum MyError {
    SqliteError(rusqlite::Error),
    BcryptError(bcrypt::BcryptError),
}

impl From<rusqlite::Error> for MyError {
    fn from(error: rusqlite::Error) -> Self {
        MyError::SqliteError(error)
    }
}

impl From<bcrypt::BcryptError> for MyError {
    fn from(error: bcrypt::BcryptError) -> Self {
        MyError::BcryptError(error)
    }
}

pub fn add_user(user: &User, password: &str) -> Result<(), MyError> {
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    let conn = rusqlite::Connection::open("expenses.db")?;
    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        rusqlite::params![user.username, password_hash],
    )?;
    Ok(())
}

pub fn authenticate_user(username: &str, password: &str) -> Result<Option<User>> {
    let conn = Connection::open("expenses.db")?;
    if let Ok(mut stmt) =
        conn.prepare("SELECT id, username, password_hash FROM users WHERE username = ?1")
    {
        if let Some(row) = stmt
            .query_row(params![username], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .optional()?
        {
            let (user_id, user_name, password_hash): (i32, String, String) = row;
            // Correctly handle bcrypt errors
            match verify(password, &password_hash) {
                Ok(valid) => {
                    if valid {
                        return Ok(Some(User {
                            id: user_id,
                            username: user_name,
                            password_hash: password_hash,
                        }));
                    }
                }
                Err(_) => {
                    // Handle bcrypt error (e.g., log it or return a specific error)
                }
            }
        }
    }
    Ok(None)
}
