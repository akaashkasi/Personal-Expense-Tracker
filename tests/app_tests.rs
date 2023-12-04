use expense_tracker::ui::MyApp;
#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add_expense() {
        let egui_ctx = create_test_egui_context(); // Create a test context for egui
        let mut app = MyApp::new(&egui_ctx);

        app.expense_name = "Coffee".to_string();
        app.expense_amount = "3.50".to_string();
        app.expense_date = "2023-11-07".to_string();
        app.category = "Beverage".to_string();
        app.payment_method = "Cash".to_string();

        // Assuming add_expense_to_db modifies the state directly
        app.add_expense_to_db(&egui_ctx);

        assert_eq!(app.expenses.len(), 1);
        assert_eq!(app.expenses[0].description, "Coffee");
        assert_eq!(app.expenses[0].amount, 3.50);
        assert!(app.expense_name.is_empty());
        assert!(app.expense_amount.is_empty());
        assert!(app.category.is_empty());

        app.delete_expense_from_db(app.expenses[0].id); //cleans the app.expenses vector
    }
}
