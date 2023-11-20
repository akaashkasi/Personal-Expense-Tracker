use crate::models;
use crate::models::{add_expense, Expense};
use crate::ui;
use eframe::egui;

pub struct MyApp {
    pub expense_name: String,
    pub expense_amount: String,
    pub payment_method: String,
    pub category: String,
    pub expenses: Vec<Expense>,
    pub expense_date: String,
    pub warning_message: Option<String>,
}

impl MyApp {
    pub fn new() -> Self {
        let mut app = MyApp {
            expense_name: String::new(),
            expense_amount: String::new(),
            payment_method: String::new(),
            category: String::new(),
            expenses: Vec::new(),
            expense_date: String::new(),
            warning_message: None,
        };
        app.load_expenses();
        app
    }

    fn load_expenses(&mut self) {
        self.expenses = models::get_expenses().unwrap_or_default();
    }

    pub fn add_expense_to_db(&mut self) {
        let amount = self.expense_amount.parse::<f32>().unwrap_or(0.0);

        let expense = Expense {
            id: 0,
            date: self.expense_date.clone(),
            amount,
            category: self.category.clone(),
            description: self.expense_name.clone(),
            payment_method: self.payment_method.clone(),
        };

        if let Err(e) = add_expense(&expense) {
            eprintln!("Failed to add expense: {}", e);
        }
        self.load_expenses();
        self.expense_name.clear();
        self.expense_amount.clear();
        self.payment_method.clear();
        self.category.clear();
    }

    pub fn delete_expense_from_db(&mut self, expense_id: i32) {
        if let Err(e) = models::delete_expense(expense_id) {
            eprintln!("Failed to delete expense: {}", e);
        }
        self.load_expenses();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::render_ui(ctx, self);
    }
}
