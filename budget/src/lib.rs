use std::collections::HashMap;
use std::io::ErrorKind;
use std::fs;

use toml::de::Error as DeserializerError;
use serde::{Deserialize, Deserializer};
use chrono::NaiveDate;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Account {
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDate,
    pub budget: f64,
    #[serde(default)]
    pub essential_categories: Vec<String>,
    pub days: Vec<Day>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Day {
    #[serde(deserialize_with = "deserialize_date")]
    pub date: NaiveDate,
    #[serde(default)]
    pub expenses: Vec<Expense>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Expense {
    pub name: String,
    pub price: f64,
    #[serde(default = "shared_qty_default")]
    pub qty: u32, // unused for now, might use it the future or remove it
    #[serde(default = "shared_qty_default")]
    pub shared: u32,
    #[serde(default = "recurring_default")]
    pub recurring: bool,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Calculated {
    pub all_day_average: f64,
    pub essential_day_average: f64,
    pub categories_day_average: HashMap<String, f64>,
    pub essential_subtotal: f64,
    pub categories_subtotal: HashMap<String, f64>,
    pub total: f64,
    pub balance: f64,
    pub days_left: f64,
    pub days_left_essential: f64,
    pub last_day: NaiveDate,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    IOError(ErrorKind),
    DeserializerError(DeserializerError),
}

fn shared_qty_default() -> u32 {
    1
}

fn recurring_default() -> bool {
    false
}

// Parse the dates from toml's Datetime to Chrono's NaiveDate
// Probably unnecessary for now, but since I am planning on using the dates in
// the future to more easily count the days, it would be better to have them in
// a proper format
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where D: Deserializer<'de> {
    toml::value::Datetime::deserialize(deserializer)
        .map(|v| {
            let s = v.to_string();

            NaiveDate::parse_from_str(&s, "%Y-%m-%d")
        })?
        .map_err(serde::de::Error::custom)
}

pub fn parse_account(path: &str) -> Result<Account, ParseError> {
    let contents = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(error) => {
            return Err(ParseError::IOError(error.kind()));
        },
    };

    match toml::from_str::<Account>(&contents) {
        Ok(budget) => Ok(budget),
        Err(error) => Err(ParseError::DeserializerError(error)),
    }
}

pub fn calculate(account: &Account) -> Option<Calculated> {
    if account.days.len() < 1 {
        return None;
    }

    let mut calculated = Calculated {
        all_day_average: 0.0,
        essential_day_average: 0.0,
        categories_day_average: HashMap::<String, f64>::new(),
        essential_subtotal: 0.0,
        categories_subtotal: HashMap::<String, f64>::new(),
        total: 0.0,
        balance: 0.0,
        days_left: 0.0,
        days_left_essential: 0.0,
        last_day: account.days.last().unwrap().date,
    };

    for day in account.days.iter() {
        if day.date > calculated.last_day {
            calculated.last_day = day.date;
        }

        for expense in day.expenses.iter() {
            calculated.total += expense.price;

            if let Some(category) = &expense.category {
                if let Some(category_subtotal) = 
                calculated.categories_subtotal.get_mut(category) {
                    *category_subtotal += expense.price;
                } else {
                    calculated.categories_subtotal.insert(
                        category.to_string(),
                        expense.price,
                    );
                }

                if account.essential_categories.contains(category) {
                    calculated.essential_subtotal += expense.price;
                }
            }
        }
    }

    let days_elapsed = 
        (calculated.last_day - account.start_date).num_days() + 1;

    calculated.all_day_average = calculated.total / days_elapsed as f64;
    calculated.essential_day_average = 
        calculated.essential_subtotal / days_elapsed as f64;

    for (category, subtotal) in calculated.categories_subtotal.iter() {
        calculated.categories_day_average
            .insert(
                category.clone(),
                subtotal / days_elapsed as f64,
            );
    }

    calculated.balance = account.budget - calculated.total;

    calculated.days_left = calculated.balance / calculated.all_day_average;
    calculated.days_left_essential = 
        calculated.balance / calculated.essential_day_average;

    Some(calculated)
}
