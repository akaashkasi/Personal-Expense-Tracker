mod app;
mod models;
mod ui;

fn main() {
    let options = eframe::NativeOptions::default();
    match models::create_expense_table() {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Failed to create table: {}", e),
    }
    eframe::run_native(
        "Expense Tracker",
        options,
        Box::new(|_| Box::new(app::MyApp::new())),
    );
}
