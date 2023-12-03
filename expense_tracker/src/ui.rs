pub use crate::app::MyApp;
use eframe::egui;
use eframe::egui::{Color32, TextureOptions};
use image::GenericImageView;
use image::{io::Reader as ImageReader, ImageError};
use plotters::prelude::*;
use plotters::style::{Color, ShapeStyle};
use std::collections::HashMap;

const ORANGE: RGBColor = RGBColor(255, 165, 0);
const PURPLE: RGBColor = RGBColor(128, 0, 128);
const PINK: RGBColor = RGBColor(255, 192, 203);
const LIME_GREEN: RGBColor = RGBColor(50, 205, 50);
const INDIGO: RGBColor = RGBColor(75, 0, 130);

pub fn load_image_to_memory(file_path: &str) -> Result<(Vec<u8>, [usize; 2]), ImageError> {
    let img = ImageReader::open(file_path)?.decode()?;
    let dimensions = img.dimensions();
    Ok((
        img.to_rgba8().into_raw(),
        [dimensions.0 as usize, dimensions.1 as usize],
    ))
}

pub fn load_texture_from_memory(
    egui_ctx: &egui::Context,
    image_data: &[u8],
    size: [usize; 2],
    texture_id: String
) -> egui::TextureHandle {
    let image = egui::ColorImage::from_rgba_unmultiplied(size, image_data);
    let image_data: egui::ImageData = image.into();
    egui_ctx.load_texture(&texture_id, image_data, TextureOptions::default())
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



pub fn create_monthly_spending_chart(
    data: &HashMap<String, f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating monthly spending chart..."); // Diagnostic message
    println!("Data received for chart: {:?}", data);
    let root_area = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut midpoints: Vec<(f64, String)> = Vec::new();

    let total_expenses: f32 = data.values().sum();
    let colors = vec![RED, BLUE, GREEN, YELLOW, CYAN, MAGENTA, ORANGE, PINK, PURPLE, LIME_GREEN, INDIGO];

    let center = (320, 240);
    let radius = 150.0;
    let mut start_angle_degrees = -90.0 as f64;  // Start from the top of the circle

    // If there's only one entry, draw a full circle
    if data.len() == 1 {
        let category = data.keys().next().unwrap(); // Get the single category
        let filled_style = ShapeStyle {
            color: colors[0].to_rgba().mix(0.8).into(), // Use the first color
            filled: true,
            stroke_width: 2,
        };
        root_area.draw(&Circle::new(center, radius, filled_style))?;
    } else {
        // Draw segments for each category
        for (index, (category, &amount)) in data.iter().enumerate() {
            println!("Processing category: {}, amount: {}", category, amount); // Diagnostic message
            let fraction = (amount / total_expenses) as f64;
            let sweep = fraction * 360.0; // Degrees of the circle that this category occupies
            let end_angle_degrees = start_angle_degrees + sweep;
            let end_angle = end_angle_degrees.to_radians(); // Convert to radians for the drawing
            
            // Define the color for this segment
            let color = colors[index % colors.len()].to_rgba().mix(0.8).into();
            
            // Create a filled style for the pie segment
            let filled_style = ShapeStyle {
                color: color,
                filled: true,
                stroke_width: 2,
            };
            let midpoint_angle_degrees = start_angle_degrees + sweep / 2.0;
            midpoints.push((midpoint_angle_degrees, category.clone()));
            
            // Draw the pie segment
            root_area.draw(&PathElement::new(
                vec![
                    center,
                    polar_to_cartesian(center, radius, start_angle_degrees.to_radians()),
                    polar_to_cartesian(center, radius, end_angle),
                    center,
                ],
                filled_style,
            ))?;
            
            start_angle_degrees = end_angle_degrees; // Set up the start angle for the next segment
        }

        for (angle_degrees, category) in midpoints {
            // Convert the midpoint angle to radians and calculate label position
            let label_angle = angle_degrees.to_radians();
            let (label_x, label_y) = polar_to_cartesian(center, radius + 20.0 /* label offset */, label_angle);
        
            // Create a TextStyle
            let text_style = TextStyle::from(("Arial", 15).into_font()).color(&BLACK);
        
            // Draw the category label
            root_area.draw_text(
                &category,
                &text_style,
                (label_x, label_y),
            )?;
        }
    }
    println!("Finished drawing chart."); // Diagnostic message
    root_area.present()?;
    println!("Chart saved to 'chart.png'."); // Diagnostic message
    Ok(())
}

fn polar_to_cartesian(center: (i32, i32), radius: f64, angle_in_radians: f64) -> (i32, i32) {
    let (cx, cy) = center;
    let x = (angle_in_radians.cos() * radius) as i32 + cx;
    let y = (angle_in_radians.sin() * radius) as i32 + cy;
    (x, y)
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
