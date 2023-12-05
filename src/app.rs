use crate::models;
use crate::models::add_user;
use crate::models::{add_expense, Expense, User};
use crate::ui;
use crate::ui::create_monthly_spending_chart;
use crate::ui::load_texture_from_memory;
use eframe::egui;
use image::{io::Reader as ImageReader, GenericImageView};
use std::collections::HashMap;

pub fn load_image_to_memory(file_path: &str) -> Result<(Vec<u8>, [u32; 2]), image::ImageError> {
    let img = ImageReader::open(file_path)?.decode()?;
    let dimensions = img.dimensions();
    Ok((img.to_rgba8().into_raw(), [dimensions.0, dimensions.1]))
}

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
    pub image_texture: Option<egui::TextureHandle>,
    pub show_monthly_trends: bool,
    pub show_yearly_comparison: bool,
    pub show_monthly_spending: bool,
}

impl MyApp {
    pub fn new(egui_ctx: &egui::Context) -> Self {
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
            image_texture: None,
            show_monthly_trends: false,
            show_yearly_comparison: false,
            show_monthly_spending: false,
        };
        app.load_expenses();
        app.update_monthly_spending_chart(egui_ctx);
        app
    }

    pub fn calculate_category_totals(&self) -> HashMap<String, f32> {
        let mut category_totals = HashMap::new();
        for expense in &self.expenses {
            let amount = category_totals
                .entry(expense.category.clone())
                .or_insert(0.0);
            *amount += expense.amount;
        }
        category_totals
    }

    pub fn update_chart(&mut self, egui_ctx: &egui::Context) {
        self.update_monthly_spending_chart(egui_ctx); // This already recalculates and updates the chart
        egui_ctx.request_repaint(); // Request the UI to repaint after updating the chart
    }

    // In MyApp struct in app.rs
    pub fn update_monthly_spending_chart(&mut self, egui_ctx: &egui::Context) {
        let monthly_spending = self.calculate_category_totals();
        match create_monthly_spending_chart(&monthly_spending) {
            Ok(_) => {
                if let Ok((image_data, image_size)) = load_image_to_memory("chart.png") {
                    let image_size_usize = [image_size[0] as usize, image_size[1] as usize];

                    // Ensure we use a unique identifier for the texture to avoid caching issues
                    let texture_id = format!(
                        "chart_texture_{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                    );

                    // Dispose of the old texture and create a new one
                    let texture_id_clone = texture_id.clone();
                    self.image_texture = Some(load_texture_from_memory(
                        egui_ctx,
                        &image_data,
                        image_size_usize,
                        texture_id_clone,
                    ));
                }
            }
            Err(e) => eprintln!("Error creating chart: {:?}", e),
        }
        egui_ctx.request_repaint();
    }

    pub fn process_login(&mut self, username: &str, password: &str) {
        self.warning_message = None;
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

    fn is_password_valid(&self, password: &str) -> bool {
        let has_number = password.chars().any(|c| c.is_digit(10));
        let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
        let has_min_length = password.len() >= 5;

        has_number && has_symbol && has_min_length
    }

    pub fn process_signup(&mut self) {
        self.warning_message = None;

        // First, check if the username and password fields are not empty
        if self.new_username.is_empty() || self.new_password.is_empty() {
            self.warning_message = Some("Username and password cannot be empty".to_string());
            return;
        }

        // Then, check if the username already exists
        match models::is_username_unique(&self.new_username) {
            Ok(false) => {
                self.warning_message = Some("Username already exists".to_string());
                return;
            }
            Err(_) => {
                self.warning_message = Some("Failed to check username uniqueness".to_string());
                return;
            }
            Ok(true) => {
                // Username is unique, now validate password complexity
                if !self.is_password_valid(&self.new_password) {
                    self.warning_message = Some("Password must be at least 5 characters long, include a number and a symbol".to_string());
                    return;
                }

                // Proceed with adding the user
                let user = User {
                    id: 0, // Or generate an ID as needed
                    username: self.new_username.clone(),
                    password_hash: String::new(), // This will be set in add_user
                };
                match add_user(&user, &self.new_password) {
                    Ok(_) => {
                        self.warning_message = Some("User successfully registered!".to_string())
                    }
                    Err(e) => self.warning_message = Some(format!("Failed to register: {:?}", e)),
                }
            }
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

    pub fn add_expense_to_db(&mut self, egui_ctx: &egui::Context) {
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
        self.update_monthly_spending_chart(egui_ctx);
        self.update_chart(egui_ctx);
    }

    pub fn delete_expense_from_db(&mut self, expense_id: i32, egui_ctx: &egui::Context) {
        if let Err(e) = models::delete_expense(expense_id) {
            eprintln!("Failed to delete expense: {}", e);
        }
        self.load_expenses();
        self.update_monthly_spending_chart(egui_ctx);
        self.update_chart(egui_ctx);
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
