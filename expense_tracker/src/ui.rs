use crate::app::MyApp;
use eframe::egui;
use eframe::egui::Color32;

pub fn render_login_ui(ui: &mut egui::Ui, app: &mut MyApp) {
    let username = app.username.clone();
    let password = app.password.clone();
    ui.vertical_centered(|ui| {
        ui.heading("Login to Expense Tracker");
        ui.horizontal(|ui| {
            ui.label("Username:");
            styled_text_edit(ui, &mut app.username);
            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut app.password).password(true));
        });
        if ui.button("Login").clicked() {
            // Implement login logic in MyApp
            app.process_login(&username, &password);
        }
    });
    display_warning_message(ui, app);
}

pub fn render_signup_ui(ui: &mut egui::Ui, app: &mut MyApp) {
    ui.vertical_centered(|ui| {
        ui.heading("Sign Up for Expense Tracker");

        ui.horizontal(|ui| {
            ui.label("New Username:");
            styled_text_edit(ui, &mut app.new_username);
        });

        ui.horizontal(|ui| {
            ui.label("New Password:");
            // Obfuscate password input
            ui.add(egui::TextEdit::singleline(&mut app.new_password).password(true));
        });

        if ui.button("Sign Up").clicked() {
            app.process_signup();
        }
    });
    display_warning_message(ui, app);
}

pub fn styled_text_edit(ui: &mut egui::Ui, text: &mut String) {
    ui.scope(|ui| {
        let mut style = (*ui.ctx().style()).clone();
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_gray(240); // Custom background color
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::none(); // Remove border
        ui.ctx().set_style(style);

        ui.text_edit_singleline(text);
    });
}

// Function to display warning message
fn display_warning_message(ui: &mut egui::Ui, app: &MyApp) {
    if let Some(warning) = &app.warning_message {
        ui.colored_label(Color32::RED, warning);
    }
}

pub fn render_expense_tracker_ui(_ui: &mut egui::Ui, app: &mut MyApp, ctx: &egui::Context) {
    let mut style: egui::Style = (*ctx.style()).clone();

    // Define theme colors
    let primary_color = Color32::from_rgb(100, 149, 237); // Cornflower blue
    let lighter_primary_color = Color32::from_rgb(130, 179, 255); // Lighter shade of primary color
    let secondary_color = Color32::from_rgb(245, 245, 245); // White smoke

    // General styling
    style.visuals.widgets.noninteractive.bg_fill = secondary_color;
    style.visuals.widgets.noninteractive.fg_stroke.color = Color32::BLACK;
    style.visuals.widgets.active.bg_fill = primary_color;
    style.visuals.widgets.active.fg_stroke.color = Color32::WHITE;
    style.visuals.widgets.hovered.bg_fill = lighter_primary_color;
    style.visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;

    // Button styling
    style.visuals.button_frame = true;
    style.visuals.widgets.open.rounding = egui::Rounding::from(4.0); // Apply rounding to buttons

    // Text styling
    style.visuals.override_text_color = Some(Color32::BLACK);

    ctx.set_style(style);
    let mut expenses_to_delete: Vec<i32> = Vec::new();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Personal Expense Tracker");

        // Add UI elements here
        ui.horizontal(|ui| {
            let total_expenses: f32 = app.expenses.iter().map(|expense| expense.amount).sum();
            // Display dynamic total expenses
            ui.label(format!("Total Expenses: ${:.2}", total_expenses)); // Replace with dynamic content
        });

        ui.separator();
        if ui.button("Logout").clicked() {
            app.logout();
        }
        ui.vertical(|ui| {
            ui.heading("Add New Expense");
            ui.horizontal(|ui| {
                ui.label("New Expense Name:");
                ui.text_edit_singleline(&mut app.expense_name);
            });
            ui.horizontal(|ui| {
                ui.label("New Expense Amount:");
                ui.text_edit_singleline(&mut app.expense_amount);
            });
            ui.horizontal(|ui| {
                ui.label("Date:");
                ui.text_edit_singleline(&mut app.expense_date);
            });
            ui.horizontal(|ui| {
                ui.label("Category:");
                ui.text_edit_singleline(&mut app.category);
            });
            ui.horizontal(|ui| {
                ui.label("Payment Method:");
                let payment_methods = ["Cash", "Card"];
                egui::ComboBox::from_label("")
                    .selected_text(app.payment_method.clone())
                    .show_ui(ui, |ui| {
                        for payment_method in payment_methods.iter() {
                            ui.selectable_value(
                                &mut app.payment_method,
                                payment_method.to_string(),
                                *payment_method,
                            );
                        }
                    });
            });

            let add_button = ui.add(egui::Button::new("Add"));

            if add_button.clicked() {
                if !app.expense_name.is_empty()
                    && !app.expense_amount.is_empty()
                    && !app.expense_date.is_empty()
                    && !app.category.is_empty()
                    && !app.payment_method.is_empty()
                {
                    app.add_expense_to_db();
                    app.warning_message = None; // Clear any previous warning
                } else {
                    app.warning_message = Some("Please fill in all fields".to_string());
                }
            }

            if let Some(warning) = &app.warning_message {
                ui.colored_label(egui::Color32::RED, warning);
            }
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.heading("Expenses List");

            // Improved table of expenses
            egui::Grid::new("expenses_table")
                .num_columns(6) // Adjust the number of columns to include all properties
                .striped(true)
                .show(ui, |ui| {
                    // Header row
                    ui.label("Date");
                    ui.label("Name");
                    ui.label("Amount");
                    ui.label("Category");
                    ui.label("Payment Method");
                    ui.label(""); // Placeholder for the delete button column
                    ui.end_row();

                    // Rows for each expense
                    for expense in &app.expenses {
                        ui.label(&expense.date);
                        ui.label(&expense.description);
                        ui.label(&format!("{:.2}", expense.amount));
                        ui.label(&expense.category);
                        ui.label(&expense.payment_method);
                        if ui.button("Delete").clicked() {
                            expenses_to_delete.push(expense.id);
                        }
                        ui.end_row();
                    }
                });
        });
    });

    // Process deletions after UI rendering
    for id in expenses_to_delete {
        app.delete_expense_from_db(id);
    }
}
