use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use toml::de::Error as DeserializerError;

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
	#[serde(default)]
	/// Whom this expense is shared with (if anybody).
	pub shared: Vec<String>,
	#[serde(default)]
	/// Whether this was something we paid for somebody else, and thus is owed
	/// to us. If true, then shared is the list of person(s) that owe us this
	/// expense, and should therefore contain at least one name.
	pub owed: bool,
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
	pub owed: HashMap<String, f64>,
	pub total_owed: f64,
	pub days_left: f64,
	pub days_left_essential: f64,
	pub last_day: NaiveDate,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
	IOError(ErrorKind),
	DeserializerError(DeserializerError),
}

// Parse the dates from toml's Datetime to Chrono's NaiveDate
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
	D: Deserializer<'de>,
{
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
		}
	};

	match toml::from_str::<Account>(&contents) {
		Ok(budget) => Ok(budget),
		Err(error) => Err(ParseError::DeserializerError(error)),
	}
}

pub fn calculate(account: &Account, consider_owed: bool) -> Option<Calculated> {
	if account.days.is_empty() {
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
		owed: HashMap::<String, f64>::new(),
		total_owed: 0.0,
		days_left: 0.0,
		days_left_essential: 0.0,
		last_day: account.days.last().unwrap().date,
	};

	for day in account.days.iter() {
		if day.date > calculated.last_day {
			calculated.last_day = day.date;
		}

		for expense in day.expenses.iter() {
			let mut actual_expense: f64 = 0.0;

			if expense.shared.len() > 0 {
				let owed_share = if expense.owed {
					expense.price / expense.shared.len() as f64
				} else {
					actual_expense =
						expense.price / (expense.shared.len() as f64 + 1.0);
					actual_expense
				};

				for person in expense.shared.iter() {
					calculated.total_owed += owed_share;

					if let Some(owed_by_person) =
						calculated.owed.get_mut(person)
					{
						*owed_by_person += owed_share;
					} else {
						calculated.owed.insert(person.clone(), owed_share);
					}
				}
			} 

			if expense.shared.len() == 0 || consider_owed {
				actual_expense = expense.price;
			} else if expense.owed {
				continue;
			}

			calculated.total += actual_expense;

			if let Some(category) = &expense.category {
				if let Some(category_subtotal) =
					calculated.categories_subtotal.get_mut(category)
				{
					*category_subtotal += actual_expense;
				} else {
					calculated
						.categories_subtotal
						.insert(category.to_string(), actual_expense);
				}

				if account.essential_categories.contains(category) {
					calculated.essential_subtotal += actual_expense;
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
		calculated
			.categories_day_average
			.insert(category.clone(), subtotal / days_elapsed as f64);
	}

	calculated.balance = account.budget - calculated.total;

	calculated.days_left = calculated.balance / calculated.all_day_average;
	calculated.days_left_essential =
		calculated.balance / calculated.essential_day_average;

	Some(calculated)
}
