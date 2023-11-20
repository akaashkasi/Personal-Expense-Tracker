use expense_tracker::models::Expense; // Import the Expense struct from the models module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expense_creation() {
        // Test the creation of an Expense object
        let expense = Expense {
            id: 0, // Assuming 'id' is not critical for this test
            date: "2023-11-07".to_string(),
            amount: 50.0,
            category: "Groceries".to_string(),
            description: "Weekly groceries".to_string(),
            payment_method: "Credit Card".to_string(),
        };

        // Assert that the expense has the correct properties
        assert_eq!(expense.category, "Groceries");
        assert_eq!(expense.amount, 50.0);
    }

    #[test]
    fn test_expense_total() {
        // Test the calculation of the total amount spent on expenses
        let expense1 = Expense {
            id: 0,
            date: "".to_string(),
            amount: 50.0,
            category: "Groceries".to_string(),
            description: "".to_string(),
            payment_method: "".to_string(),
        };
        let expense2 = Expense {
            id: 0,
            date: "".to_string(),
            amount: 20.0,
            category: "Transportation".to_string(),
            description: "".to_string(),
            payment_method: "".to_string(),
        };
        let expense3 = Expense {
            id: 0,
            date: "".to_string(),
            amount: 30.0,
            category: "Entertainment".to_string(),
            description: "".to_string(),
            payment_method: "".to_string(),
        };

        let total = expense1.amount + expense2.amount + expense3.amount;

        // Assert that the total amount is calculated correctly
        assert_eq!(total, 100.0);
    }
}


// Path: expense_tracker/tests/models_tests.rs