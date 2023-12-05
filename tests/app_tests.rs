use expense_tracker::models;
use expense_tracker::models::Expense;
use expense_tracker::ui::MyApp;

fn create_test_app() -> MyApp {
    MyApp {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app_has_no_expenses() {
        let app = create_test_app();
        assert!(app.expenses.is_empty());
    }

    #[test]
    fn test_adding_expense_increases_count() {
        let mut app = create_test_app();
        app.expense_name = "Test Expense".to_string();
        app.expense_amount = "100.0".to_string();
        app.expense_date = "2023-01-01".to_string();
        app.category = "Food".to_string();
        app.payment_method = "Cash".to_string();

        // Mock the functionality that adds the expense to the database
        // For this example, we'll just add it to the expenses vector
        app.expenses.push(Expense {
            id: 1,
            date: app.expense_date.clone(),
            amount: app.expense_amount.parse::<f32>().unwrap(),
            category: app.category.clone(),
            description: app.expense_name.clone(),
            payment_method: app.payment_method.clone(),
        });

        assert_eq!(app.expenses.len(), 1);
    }

    #[test]
    fn test_calculate_category_totals() {
        let mut app = create_test_app();
        app.expenses = vec![
            Expense {
                id: 1,
                date: "2023-01-01".to_string(),
                amount: 100.0,
                category: "Food".to_string(),
                description: "Groceries".to_string(),
                payment_method: "Cash".to_string(),
            },
            Expense {
                id: 2,
                date: "2023-01-02".to_string(),
                amount: 150.0,
                category: "Food".to_string(),
                description: "Restaurant".to_string(),
                payment_method: "Card".to_string(),
            },
        ];

        let category_totals = app.calculate_category_totals();
        assert_eq!(category_totals.get("Food"), Some(&250.0));
    }

    #[test]
    fn test_user_signup_successful() {
        let mut app = create_test_app();

        // Mock the user registration process
        app.new_username = "newuser2".to_string();
        app.new_password = "newpass123!".to_string(); // Assuming this meets your password criteria
        app.process_signup();

        // Check the warning message for successful registration
        assert_eq!(
            app.warning_message,
            Some("User successfully registered!".to_string())
        );

        // Cleanup: Remove the test user from the database
        // This assumes you have a function to delete a user and it returns a Result type
        if let Err(e) = models::delete_user("newuser2") {
            eprintln!("Failed to clean up test user: {}", e);
        }
    }

    #[test]
    fn test_user_signup_unsuccessful() {
        let mut app = create_test_app();

        // Mock the user registration process
        app.new_username = "newuser4".to_string();
        app.new_password = "newpass".to_string(); // Assuming this meets your password criteria
        app.process_signup();

        // Check the warning message for successful registration
        assert_eq!(
            app.warning_message,
            Some("Password must be at least 5 characters long, include a number and a symbol".to_string())
        );
    }

    #[test]
    fn test_delete_expense() {
        let mut app = create_test_app();
        app.expenses = vec![
            Expense {
                id: 1,
                date: "2023-01-01".to_string(),
                amount: 100.0,
                category: "Food".to_string(),
                description: "Groceries".to_string(),
                payment_method: "Cash".to_string(),
            },
            Expense {
                id: 2,
                date: "2023-01-02".to_string(),
                amount: 150.0,
                category: "Food".to_string(),
                description: "Restaurant".to_string(),
                payment_method: "Card".to_string(),
            },
        ];

        // Simulate deleting an expense
        let expense_id_to_delete = 1;
        app.expenses.retain(|e| e.id != expense_id_to_delete);

        assert_eq!(app.expenses.len(), 1);
        assert!(app.expenses.iter().all(|e| e.id != expense_id_to_delete));
    }

    // Add more tests to cover other functionalities like login, signup, delete_expense_from_db, etc.
}
