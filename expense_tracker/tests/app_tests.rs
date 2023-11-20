use expense_tracker::app::MyApp;

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add_expense() {
        let mut app = MyApp::new(); // Assuming MyApp has a new() function
        app.expense_name = "Coffee".to_string();
        app.expense_amount = "3.50".to_string();
        app.expense_date = "2023-11-07".to_string();
        app.category = "Beverage".to_string();
        app.payment_method = "Cash".to_string();

        app.add_expense_to_db(); // Assuming this function exists and modifies the state

        assert_eq!(app.expenses.len(), 1); // Check if an expense was added
        assert_eq!(app.expenses[0].description, "Coffee"); // Check if the expense has the correct description
        assert_eq!(app.expenses[0].amount, 3.50); // Check if the expense has the correct amount
        assert!(app.expense_name.is_empty()); // Check if the expense name was cleared
        assert!(app.expense_amount.is_empty()); // Check if the expense amount was cleared
        assert!(app.category.is_empty()); // Check if the expense category was cleared

        app.delete_expense_from_db(app.expenses[0].id); //cleans the app.expenses vector
    }
}
