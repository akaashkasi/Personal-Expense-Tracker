use crate::models;
use crate::models::add_user;
use crate::models::{add_expense, Expense, User};
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
    pub current_user_id: Option<i32>,
    pub username: String,
    pub password: String,
    pub is_logged_in: bool,
    pub current_user: Option<User>,
    pub new_username: String,
    pub new_password: String,
    pub showing_signup: bool,
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
            current_user_id: None,
            username: String::new(),
            password: String::new(),
            is_logged_in: false,
            current_user: None,
            new_username: String::new(),
            new_password: String::new(),
            showing_signup: false,
        };
        app.load_expenses();
        app
    }

    pub fn process_login(&mut self, username: &str, password: &str) {
        if let Ok(Some(user)) = models::authenticate_user(username, password) {
            self.is_logged_in = true;
            self.current_user = Some(user);
            self.load_expenses(); // Load expenses specific to this user
        } else {
            self.warning_message = Some("Invalid username or password".to_string());
        }
    }

    pub fn logout(&mut self) {
        self.is_logged_in = false;
        self.current_user = None;
        self.username.clear();
        self.password.clear();
        self.current_user_id = None;
        // Clear any other user-specific data if necessary
        self.showing_signup = true;
    }

    pub fn process_signup(&mut self) {
        if self.new_username.is_empty() || self.new_password.is_empty() {
            self.warning_message = Some("Username and password cannot be empty".to_string());
            return;
        }
        let user = User {
            id: 0, // Or generate an ID as needed
            username: self.new_username.clone(),
            password_hash: String::new(), // This will be set in add_user
        };
        match add_user(&user, &self.new_password) {
            Ok(_) => self.warning_message = Some("User successfully registered!".to_string()),
            Err(e) => self.warning_message = Some(format!("Failed to register: {:?}", e)),
        }
    }

    fn show_signup(&mut self) {
        self.showing_signup = true;
    }

    fn show_login(&mut self) {
        self.showing_signup = false;
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
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_logged_in {
                ui::render_expense_tracker_ui(ui, self, ctx);
            } else {
                // Add UI elements to toggle between login and signup
                if ui.button("Switch to Signup").clicked() {
                    self.show_signup();
                }
                if ui.button("Switch to Login").clicked() {
                    self.show_login();
                }

                if self.showing_signup {
                    ui::render_signup_ui(ui, self);
                } else {
                    ui::render_login_ui(ui, self);
                }
            }
        });
    }
}
