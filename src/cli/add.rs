use clap::{Args, Subcommand};
use std::fmt;

#[derive(Subcommand)]
pub enum Add {
    /// Add new expense
    Expense(Operation),

    /// Add new income
    Income(Operation),
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Add::Expense(_) => write!(f, "expense"),
            Add::Income(_) => write!(f, "income"),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct Operation {
    /// Absolute value (without sign)
    #[arg(short, long)]
    pub amount: f64,

    /// Transaction date in yyyy-mm-dd format
    #[arg(short, long)]
    pub date: Option<String>,

    /// Payment method used (e.g., debit, cash)
    #[arg(short, long)]
    pub payment_method: Option<String>,

    /// Main category of the operation
    #[arg(short, long)]
    pub category: Option<String>,

    /// Tags to classify into subcategories (e.g., -t bill -t internet)
    #[arg(short, long)]
    pub tags: Vec<String>,

    /// Description of the income/expense
    pub description: Option<String>,
}

pub fn add_data(add: Add) {
    let operation = add.to_string();

    match add {
        Add::Expense(expense) => run_operation(operation, expense),
        Add::Income(income) => run_operation(operation, income),
    }
}

fn run_operation(operation: String, data: Operation) {
    println!("{}", operation);
    println!("{:?}", data)
}
