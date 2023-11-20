pub struct Expense {
    pub id: i32,
    pub date: String,
    pub amount: f32,
    pub category: String,
    pub description: String,
    pub payment_method: String,
}

use rusqlite::{params, Connection, Result};

pub fn create_expense_table() -> Result<()> {
    let conn = Connection::open("expenses.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            description TEXT,
            payment_method TEXT
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
