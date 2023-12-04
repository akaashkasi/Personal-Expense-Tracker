use expense_tracker::app::MyApp;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_app() -> MyApp {
        // Setup your MyApp instance with test configuration or mock dependencies
        MyApp::new()
    }

    #[test]
    fn test_add_expense_to_db() {
        let mut app = setup_test_app();

        // Set up the state as if the user has filled the form
        app.expense_name = "Coffee".to_string();
        app.expense_amount = "3.50".to_string();
        app.expense_date = "2023-03-01".to_string();
        app.category = "Food".to_string();
        app.payment_method = "Cash".to_string();

        // Simulate adding the expense
        app.add_expense_to_db();

        // Verify that the expense was added
        assert!(!app.expenses.is_empty());
        // More detailed assertions can be added based on the structure of Expense
    }
}
