<<<<<<< HEAD
[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# Rust-based GUI for Personal Expense Tracker

## Description
The Rust Expense Tracker is a desktop application designed for personal finance management. It allows users to track their expenses by categorizing them, adding details like payment methods, and viewing them in a list or as a pie chart. The application supports user authentication, allowing each user to manage their own expenses securely. It uses SQLite for storing expense data and user accounts, ensuring data persistence across sessions.

### Key Features
- User authentication (login and signup).
- Adding, viewing, and deleting expenses.
- Categorizing expenses and viewing them as a pie chart.
- Responsive UI with expense updates triggering a chart refresh.

## Installation

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install) and its package manager, Cargo.
- Ensure you have the SQLite development files installed on your system.

### Clone the Repository
- Clone the project repository from GitHub or another source.

### Dependency Setup
- Navigate to the project directory and run `cargo build` to install the required Rust crates.

### Database Initialization
- The application automatically creates the necessary SQLite tables (`users` and `expenses`) if they don't exist.

## How to Use

### Starting the Application
- Run the application using `cargo run` from the terminal within the project directory.

### Navigating the UI
- On launch, the application presents login and signup options.
- New users can create an account through the signup page.
- Returning users can log in with their credentials.

### Expense Management
- Once logged in, users can add new expenses by specifying details like amount, category, and payment method.
- Users can view a list of their expenses and delete any unwanted entries.

### Viewing Monthly Spending
- The application generates and updates a pie chart based on the categorized expenses, providing a visual representation of spending patterns.

### Logging Out and Account Switching
- Users can log out, which clears their session and returns to the login/signup screen.

## Notes
- The application is designed to be user-friendly, prioritizing ease of use and straightforward navigation.
- It provides a simple yet effective tool for individuals looking to keep track of their expenses and gain insights into their spending habits.

## Extensibility
The application's modular design allows for easy extension and modification. Developers can add new features, enhance the UI, or integrate additional data sources as needed.