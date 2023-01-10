use clap::{Args, Subcommand};
use std::fmt;
use std::num::ParseFloatError;
use time::format_description::well_known::Iso8601;
use time::{Date, OffsetDateTime};

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
    #[arg(short, long, value_parser = parse_float)]
    pub amount: f64,

    /// Transaction date in ISO8601 format
    #[arg(short, long, value_parser = parse_date, default_value_t = default_date())]
    pub date: Date,

    /// Main category of the operation
    #[arg(short, long)]
    pub category: String,

    /// Tags to classify into subcategories (e.g., -t bill -t internet)
    #[arg(short, long)]
    pub tags: Vec<String>,

    /// Payment method used (e.g., debit, cash)
    #[arg(short, long)]
    pub payment_method: Option<String>,

    /// Description of the income/expense
    pub description: Option<String>,
}

fn parse_float(arg: &str) -> Result<f64, ParseFloatError> {
    let value = arg.trim().parse::<f64>()?;

    let value = format!("{:.2}", value).parse::<f64>()?;

    Ok(value)
}

fn parse_date(arg: &str) -> Result<Date, time::error::Parse> {
    Date::parse(arg, &Iso8601::DEFAULT)
}

fn default_date() -> Date {
    OffsetDateTime::now_local().unwrap().date()
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
