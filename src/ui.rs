use crate::app::load_image_to_memory;
pub use crate::app::MyApp;
use crate::models::Expense;
use chrono::NaiveDate;
use eframe::egui;
use eframe::egui::{Color32, TextureOptions};
use plotters::prelude::*;
use std::collections::HashMap;
use std::error::Error;

const ORANGE: RGBColor = RGBColor(255, 165, 0);
const PURPLE: RGBColor = RGBColor(128, 0, 128);
const PINK: RGBColor = RGBColor(255, 192, 203);
const LIME_GREEN: RGBColor = RGBColor(50, 205, 50);
const INDIGO: RGBColor = RGBColor(75, 0, 130);

pub fn load_texture_from_memory(
    egui_ctx: &egui::Context,
    image_data: &[u8],
    size: [usize; 2],
    texture_id: String,
) -> egui::TextureHandle {
    let image = egui::ColorImage::from_rgba_unmultiplied(size, image_data);
    let image_data: egui::ImageData = image.into();
    egui_ctx.load_texture(texture_id, image_data, TextureOptions::default())
}

const EXPENSE_CATEGORIES: &[&str] = &[
    "Housing and Utilities",
    "Food",
    "Transportation",
    "Health and Personal Care",
    "Entertainment and Leisure",
    "Shopping",
    "Education and Professional Development",
    "Travel",
    "Savings and Investments",
    "Debt Payments",
    "Miscellaneous",
];

pub fn create_monthly_spending_chart(data: &HashMap<String, f32>) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let title = "Expense Amounts by Category";

    // Specify the font style as bold and increase the font size if necessary
    let title_font = FontDesc::new(
        FontFamily::SansSerif, // You can change this to Arial or any other font if you have it
        20.0,
        FontStyle::Bold,
    )
    .color(&RGBColor(0, 123, 255)); // Example of a bold, blue color

    let title_pos = (320, 20); // Adjust as needed for your layout

    // Draw the title
    root_area.draw_text(title, &title_font, title_pos)?;

    let total_expenses: f32 = data.values().sum();
    let mut sizes = Vec::new();
    let mut empty_labels = Vec::new();
    let mut custom_labels = Vec::new();

    let colors = vec![
        RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA, ORANGE, PINK, PURPLE, LIME_GREEN, INDIGO,
    ];
    let color_iter = colors.into_iter().cycle();

    let center = (320, 240);
    let radius = 150.0;
    let label_font = FontDesc::new(FontFamily::SansSerif, 16.0, FontStyle::Normal).color(&BLACK);

    for (category, &amount) in data.iter() {
        let proportion = (amount / total_expenses) as f64;
        sizes.push(proportion * 100.0); // Proportion in percentage
        empty_labels.push(""); // Empty label for Pie::new
        let label = format!("{:.2}% {}", proportion * 100.0, category);
        custom_labels.push(label);
    }

    let color_slice: Vec<_> = color_iter.take(data.len()).collect();

    root_area.draw(&Pie::new(
        &center,
        &radius,
        &sizes,
        &color_slice,
        &empty_labels,
    ))?;

    // Draw custom labels
    for (i, label) in custom_labels.iter().enumerate() {
        // Calculate the position for each custom label
        let angle = std::f64::consts::PI * 2.0 * sizes[i] / 100.0 / 2.0
            + sizes.iter().take(i).sum::<f64>() / 100.0 * std::f64::consts::PI * 2.0;
        let label_x = center.0 as f64 + angle.cos() * radius * 0.8;
        let label_y = center.1 as f64 + angle.sin() * radius * 0.8;

        // Draw the custom label
        root_area.draw_text(label, &label_font, (label_x as i32, label_y as i32))?;
    }

    root_area.present()?;

    Ok(())
}

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
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE; // Remove border
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
            ui.label(format!("Total Expenses: ${:.2}", total_expenses)); // Display total expenses
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
                ui.add(egui::TextEdit::singleline(&mut app.expense_date).hint_text("YYYY-MM-DD"));
            });
            ui.horizontal(|ui| {
                ui.label("Expense Type:");
                ui.push_id("expense_type", |ui| {
                    egui::ComboBox::from_label("")
                        .selected_text(app.category.clone())
                        .show_ui(ui, |ui| {
                            for category in EXPENSE_CATEGORIES.iter() {
                                ui.selectable_value(
                                    &mut app.category,
                                    category.to_string(),
                                    category.to_string(),
                                );
                            }
                        });
                });
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
                    app.add_expense_to_db(ctx);
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
                    for (index, expense) in app.expenses.iter().enumerate() {
                        ui.label(&expense.date);
                        ui.label(&expense.description);
                        ui.label(&format!("{:.2}", expense.amount));
                        ui.label(&expense.category);
                        ui.label(&expense.payment_method);
                        ui.push_id(index, |ui| {
                            if ui.button("Delete").clicked() {
                                expenses_to_delete.push(expense.id);
                            }
                        });
                        ui.end_row();
                    }
                });
        });

        ui.vertical(|ui| {
            ui.heading("Expense Analytics");

            // Example: Monthly Trends
            if ui.button("Show Monthly Trends").clicked() {
                app.show_monthly_trends = true;
                app.show_yearly_comparison = false;
                app.show_monthly_spending = false;
                let monthly_data = calculate_monthly_trends(&app.expenses); // Implement this
                let _ = create_bar_chart("monthly_trends.png", &monthly_data); // Implement this
                load_and_display_chart(ui, "monthly_trends.png"); // Implement this
            }

            if ui.button("Show Yearly Comparison").clicked() {
                app.show_yearly_comparison = true;
                app.show_monthly_trends = false;
                app.show_monthly_spending = false;
                let yearly_data = calculate_yearly_comparison(&app.expenses); // Implement this
                let _ = create_line_graph("yearly_comparison.png", &yearly_data); // Implement this
                load_and_display_chart(ui, "yearly_comparison.png"); // Implement this
            }

            if ui.button("Show Monthly Spending").clicked() {
                app.show_monthly_spending = true;
                app.show_monthly_trends = false;
                app.show_yearly_comparison = false;
            }
            if app.show_monthly_trends {
                // Display the chart here
                // Load and display the chart
                load_and_display_chart(ui, "monthly_trends.png");
            } else if app.show_yearly_comparison {
                // Display yearly comparison chart
                load_and_display_chart(ui, "yearly_comparison.png");
            } else if app.show_monthly_spending {
                // Display monthly spending chart
                load_and_display_chart(ui, "monthly_spending.png"); // Assume this is the correct file path
            }
        });

        if let Some(texture) = &app.image_texture {
            // Display the image using the texture
            ui.image(texture);
            // Forcefully request a repaint
            ctx.request_repaint();
        }
    });

    // Process deletions after UI rendering
    for id in expenses_to_delete {
        app.delete_expense_from_db(id, ctx);
    }
}

// Function to calculate monthly trends (implement the logic based on your data structure)
fn calculate_monthly_trends(expenses: &[Expense]) -> HashMap<String, f32> {
    let mut monthly_totals = HashMap::new();
    for expense in expenses {
        let month = expense.date[0..7].to_string(); // Extract YYYY-MM
        *monthly_totals.entry(month).or_insert(0.0) += expense.amount;
    }
    monthly_totals
}

// Function to create a bar chart (you'll need to define this based on your needs)
fn create_bar_chart(file_path: &str, data: &HashMap<String, f32>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut sorted_data: Vec<(&String, &f32)> = data.iter().collect();
    sorted_data.sort_by_key(|&(month, _)| month);

    let categories: Vec<String> = sorted_data
        .iter()
        .map(|(month, _)| (*month).clone())
        .collect();
    let values: Vec<f32> = sorted_data.iter().map(|(_, &value)| value).collect();

    let max_value = values.iter().fold(f32::MIN, |a, &b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Monthly Spending", ("sans-serif", 40).into_font())
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0..categories.len(), 0.0..max_value)?;

    chart
        .configure_mesh()
        .x_labels(categories.len())
        .x_label_formatter(&|x| {
            if *x < categories.len() {
                categories[*x].clone()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    chart.draw_series(
        values
            .iter()
            .enumerate()
            .map(|(idx, &value)| Rectangle::new([(idx, 0.0), (idx + 1, value)], BLUE.filled())),
    )?;

    root.present()?;
    Ok(())
}

fn calculate_yearly_comparison(expenses: &[Expense]) -> HashMap<String, f32> {
    let mut yearly_totals = HashMap::new();
    for expense in expenses {
        let date = NaiveDate::parse_from_str(&expense.date, "%Y-%m-%d").unwrap();
        let year = date.format("%Y").to_string(); // Extract only the year as a string
        *yearly_totals.entry(year).or_insert(0.0) += expense.amount;
    }
    yearly_totals
}

// Function to create a line graph (you'll need to define this based on your needs)
fn create_line_graph(file_path: &str, data: &HashMap<String, f32>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Sort the data by year
    let mut sorted_data: Vec<(&String, &f32)> = data.iter().collect();
    sorted_data.sort_by_key(|&(year, _)| year);

    let years: Vec<String> = sorted_data
        .iter()
        .map(|(year, _)| (*year).clone())
        .collect();
    let values: Vec<f32> = sorted_data.iter().map(|(_, &value)| value).collect();

    let max_value = values.iter().fold(f32::MIN, |a, &b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Yearly Spending Comparison", ("sans-serif", 40).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..years.len(), 0.0..max_value)?;

    chart
        .configure_mesh()
        .x_labels(years.len())
        .x_label_formatter(&|x| {
            if *x < years.len() {
                years[*x].clone()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    // Draw line
    chart.draw_series(LineSeries::new(
        values.iter().enumerate().map(|(idx, &value)| (idx, value)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

// Function to load and display a chart in the UI
pub fn load_and_display_chart(ui: &mut egui::Ui, file_path: &str) {
    if let Ok((image_data, image_size)) = load_image_to_memory(file_path) {
        let texture = load_texture_from_memory(
            ui.ctx(),
            &image_data,
            [image_size[0] as usize, image_size[1] as usize],
            file_path.to_owned(),
        );

        // Create an Image widget with the texture ID and size
        let image_widget = egui::Image::new(&texture);

        // Add the image widget to the UI
        ui.add(image_widget);
    }
}
