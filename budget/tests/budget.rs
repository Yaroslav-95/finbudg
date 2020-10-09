use std::collections::HashMap;

use chrono::NaiveDate;

use budget::*;

#[test]
fn can_parse_account() -> Result<(), ParseError>{
    let should_be = Account {
        start_date: NaiveDate::from_ymd(2020, 10, 1),
        end_date: NaiveDate::from_ymd(2020, 10, 31),
        budget: 420.0,
        essential_categories: vec![
            String::from("products"),
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
                        qty: 1,
                        shared: 1,
                        recurring: false,
                        category: Some(String::from("supplies")),
                    },
                    Expense {
                        name: String::from("Bacon"),
                        price: 3.33,
                        qty: 1,
                        shared: 3,
                        recurring: false,
                        category: Some(String::from("products")),
                    },
                    Expense {
                        name: String::from("Yoghurt"),
                        price: 1.24,
                        qty: 2,
                        shared: 2,
                        recurring: false,
                        category: Some(String::from("products")),
                    },
                    Expense {
                        name: String::from("Onion"),
                        price: 0.15,
                        qty: 1,
                        shared: 1,
                        recurring: false,
                        category: Some(String::from("products")),
                    },
                    Expense {
                        name: String::from("Chicken"),
                        price: 2.28,
                        qty: 1,
                        shared: 2,
                        recurring: false,
                        category: Some(String::from("products")),
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
                        qty: 1,
                        shared: 1,
                        recurring: true,
                        category: Some(String::from("utilities")),
                    },
                    Expense {
                        name: String::from("Transport card"),
                        price: 6.9,
                        qty: 1,
                        shared: 1,
                        recurring: false,
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
        all_day_average: 5.6775,
        essential_day_average: 4.725,
        categories_day_average: HashMap::<String, f64>::new(),
        essential_subtotal: 18.9,
        categories_subtotal: HashMap::<String, f64>::new(),
        total: 22.71,
        balance: 397.29,
        total_owed: HashMap::<u32, f64>::new(),
        days_left: 69.9762219286658,
        days_left_essential: 84.08253968253969,
        last_day: NaiveDate::from_ymd(2020, 10, 04),
    };

    should_be.categories_day_average.insert(
        "supplies".to_string(),
        0.9525,
    );
    should_be.categories_day_average.insert(
        "products".to_string(),
        1.75,
    );
    should_be.categories_day_average.insert(
        "transport".to_string(),
        1.725,
    );
    should_be.categories_day_average.insert(
        "utilities".to_string(),
        1.25,
    );

    should_be.categories_subtotal.insert(
        "supplies".to_string(),
        3.81,
    );
    should_be.categories_subtotal.insert(
        "products".to_string(),
        7.0,
    );
    should_be.categories_subtotal.insert(
        "transport".to_string(),
        6.9,
    );
    should_be.categories_subtotal.insert(
        "utilities".to_string(),
        5.0,
    );

    should_be.total_owed.insert(
        2,
        1.7599999999999998,
    );
    should_be.total_owed.insert(
        3,
        2.22,
    );

    let account = budget::parse_account("tests/test.toml")?;
    let actually_is = budget::calculate(&account).unwrap();

    assert_eq!(actually_is, should_be);

    Ok(())
}
