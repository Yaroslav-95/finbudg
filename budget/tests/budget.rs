use std::collections::HashMap;

use chrono::NaiveDate;

use budget::*;

#[test]
fn can_parse_account() -> Result<(), ParseError> {
	let should_be = Account {
		start_date: NaiveDate::from_ymd(2020, 10, 1),
		end_date: NaiveDate::from_ymd(2020, 10, 31),
		budget: 420.0,
		essential_categories: vec![
			String::from("produce"),
			String::from("transport"),
			String::from("utilities"),
		],
		days: vec![
			Day {
				date: NaiveDate::from_ymd(2020, 10, 1),
				expenses: vec![
					Expense {
						name: String::from("Potato masher"),
						price: 3.81,
						shared: vec![],
						owed: false,
						category: Some(String::from("supplies")),
					},
					Expense {
						name: String::from("Bacon"),
						price: 3.33,
						shared: vec![
							String::from("Fox"), 
							String::from("Falco"),
						],
						owed: false,
						category: Some(String::from("produce")),
					},
					Expense {
						name: String::from("Yoghurt"),
						price: 1.24,
						shared: vec![String::from("Falco")],
						owed: true,
						category: Some(String::from("produce")),
					},
					Expense {
						name: String::from("Onion"),
						price: 0.15,
						shared: vec![],
						owed: false,
						category: Some(String::from("produce")),
					},
					Expense {
						name: String::from("Chicken"),
						price: 2.28,
						shared: vec![String::from("Fox")],
						owed: false,
						category: Some(String::from("produce")),
					},
				],
			},
			Day {
				date: NaiveDate::from_ymd(2020, 10, 4),
				expenses: Vec::<Expense>::new(),
			},
			Day {
				date: NaiveDate::from_ymd(2020, 10, 2),
				expenses: vec![
					Expense {
						name: String::from("VPS"),
						price: 5.0,
						shared: vec![],
						owed: false,
						category: Some(String::from("utilities")),
					},
					Expense {
						name: String::from("Transport card"),
						price: 6.9,
						shared: vec![],
						owed: false,
						category: Some(String::from("transport")),
					},
				],
			},
		],
	};

	let actually_is = budget::parse_account("tests/test.toml")?;

	assert_eq!(actually_is, should_be);

	Ok(())
}

#[test]
fn can_calculate() -> Result<(), ParseError> {
	let mut should_be = Calculated {
		all_day_average: 4.5275,
		essential_day_average: 3.575,
		categories_day_average: HashMap::<String, f64>::new(),
		essential_subtotal: 14.3,
		categories_subtotal: HashMap::<String, f64>::new(),
		total: 18.11,
		balance: 401.89,
		owed: HashMap::<String, f64>::new(),
		total_owed: 4.6,
		days_left: 88.76642738818333,
		days_left_essential: 112.4167832167832,
		last_day: NaiveDate::from_ymd(2020, 10, 04),
	};

	should_be
		.categories_day_average
		.insert("supplies".to_string(), 0.9525);
	should_be
		.categories_day_average
		.insert("produce".to_string(), 0.6);
	should_be
		.categories_day_average
		.insert("transport".to_string(), 1.725);
	should_be
		.categories_day_average
		.insert("utilities".to_string(), 1.25);

	should_be
		.categories_subtotal
		.insert("supplies".to_string(), 3.81);
	should_be
		.categories_subtotal
		.insert("produce".to_string(), 2.4);
	should_be
		.categories_subtotal
		.insert("transport".to_string(), 6.9);
	should_be
		.categories_subtotal
		.insert("utilities".to_string(), 5.0);

	should_be.owed.insert(String::from("Fox"), 2.25);
	should_be.owed.insert(String::from("Falco"), 2.35);

	let mut should_be_with_owed = Calculated {
		all_day_average: 5.6775,
		essential_day_average: 4.725,
		categories_day_average: HashMap::<String, f64>::new(),
		essential_subtotal: 18.9,
		categories_subtotal: HashMap::<String, f64>::new(),
		total: 22.71,
		balance: 397.29,
		owed: HashMap::<String, f64>::new(),
		total_owed: 4.6,
		days_left: 69.9762219286658,
		days_left_essential: 84.08253968253969,
		last_day: NaiveDate::from_ymd(2020, 10, 04),
	};

	should_be_with_owed
		.categories_day_average
		.insert("supplies".to_string(), 0.9525);
	should_be_with_owed
		.categories_day_average
		.insert("produce".to_string(), 1.75);
	should_be_with_owed
		.categories_day_average
		.insert("transport".to_string(), 1.725);
	should_be_with_owed
		.categories_day_average
		.insert("utilities".to_string(), 1.25);

	should_be_with_owed
		.categories_subtotal
		.insert("supplies".to_string(), 3.81);
	should_be_with_owed
		.categories_subtotal
		.insert("produce".to_string(), 7.0);
	should_be_with_owed
		.categories_subtotal
		.insert("transport".to_string(), 6.9);
	should_be_with_owed
		.categories_subtotal
		.insert("utilities".to_string(), 5.0);

	should_be_with_owed.owed.insert(String::from("Fox"), 2.25);
	should_be_with_owed.owed.insert(String::from("Falco"), 2.35);

	let account = budget::parse_account("tests/test.toml")?;
	let actually_is = budget::calculate(&account, false).unwrap();
	let actually_is_with_owed = budget::calculate(&account, true).unwrap();

	assert_eq!(actually_is, should_be);
	assert_eq!(actually_is_with_owed, should_be_with_owed);

	Ok(())
}
