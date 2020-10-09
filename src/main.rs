use clap::{
    Arg, 
    App, 
    ArgMatches,
    crate_version, 
    crate_authors, 
    crate_description
};
use chrono::Duration;
use colored::*;

use budget::*;

fn main() {
    let matches = get_cli_matches();

    let no_color = matches.occurrences_of("plain") > 0;
    let force_color = matches.occurrences_of("force-color") > 0;
    let input = matches.value_of("INPUT").unwrap();

    let account = match budget::parse_account(input) {
        Ok(data) => data,
        Err(error) => {
            match error {
                ParseError::IOError(kind) => {
                    println!("IO error while parsing: {:?}", kind);
                },
                ParseError::DeserializerError(_) => {
                    println!("Can't parse the file, invalid syntax");
                },
            }

            ::std::process::exit(1);
        }
    };
    let maybe_calculated = budget::calculate(&account);

    if no_color && !force_color {
        colored::control::set_override(false);
    } else if force_color {
        colored::control::set_override(true);
    }

    output(account, maybe_calculated);
}

fn get_cli_matches() -> ArgMatches<'static> {
    App::new("finbudg")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("plain")
            .short("p")
            .long("plain")
            .help("Don't colorize the output. Can also be set \
            with the NO_COLOR environment variable.")
            .takes_value(false))
        .arg(Arg::with_name("force-color")
            .long("force-color")
            .help("Forces colorized output even when piping. Takes \
            precedence over --plain flag and NO_COLOR environment \
            variable")
            .takes_value(false))
        .arg(Arg::with_name("INPUT")
            .help("Expenses file in toml format to calculate from.")
            .required(true)
            .index(1))
        .get_matches()
}

fn output(account: Account, maybe_calculated: Option<Calculated>) {
    println!(
        "{}",
        format!(
            "Your expenses for the period of {} - {}",
            account.start_date.format("%Y-%m-%d"),
            account.end_date.format("%Y-%m-%d"),
        ).cyan(),
    );

    let calculated = match maybe_calculated {
        Some(data) => data,
        None => {
            println!();
            println!("{}", "You have no expenses...".italic());

            ::std::process::exit(0);
        }
    };

    let days_until_end = account.end_date - calculated.last_day;

    println!(
        "{}", 
        format!(
            "Last day on entry: {}",
            calculated.last_day.format("%Y-%m-%d"),
        ).cyan(),
    );

    println!(
        "{}", 
        format!(
            "Days until period end: {}",
            days_until_end.num_days(),
        ).cyan(),
    );

    if days_until_end < Duration::zero() {
        println!();
        println!(
            "{}", 
            "Your last day on entry is set after the last date of the period!"
            .yellow(),
        );
        println!();
    }

    println!(
        "{}",
        format!(
            "Budget: {:.2}",
            account.budget,
        ).cyan(),
    );

    println!();

    for (category, expenses) in calculated.categories_day_average.iter() {
        println!(
            "Average per day in {}: {:.2}",
            category,
            expenses,
        );
    }

    println!(
        "Average per day in essential expenses: {:.2}",
        calculated.essential_day_average,
    );

    println!(
        "Average per day: {:.2}",
        calculated.all_day_average,
    );

    println!();

    for (category, expenses) in calculated.categories_subtotal.iter() {
        println!(
            "Total in {}: {:.2}",
            category,
            expenses,
        );
    }

    println!(
        "Total in essential expenses: {:.2}",
        calculated.essential_subtotal,
    );

    println!(
        "Total: {:.2}",
        calculated.total,
    );

    println!();

    let balance_output = format!("{:.2}", calculated.balance);
    let balance_output = if calculated.balance > 0.0 {
        if account.budget / calculated.balance < 10.0 {
            balance_output.green()
        } else {
            balance_output.yellow()
        }
    } else {
        balance_output.red()
    };

    println!("Left on balance: {}", balance_output);

    println!();

    for (n, owed) in calculated.total_owed.iter() {
        println!(
            "{} person(s) owe you in shared expenses: {:.2}",
            n - 1,
            owed,
        );

        if *n > 2 {
            println!("Each owes you: {}", *owed / (*n as f64 - 1.0));
        }

        println!();
    }

    println!("Days until balance runs out:");

    let days_left_output = format!(
        "{:.2}",
        calculated.days_left,
    );
    let days_left_essential_output = format!(
        "{:.2}",
        calculated.days_left_essential,
    );

    // TODO: also show much money would be left by the end of the period

    let mut all_are_healthy = true;
    let mut essential_are_healthy = true;

    let days_left_output = 
        if days_until_end.num_days() as f64 <= calculated.days_left {
            days_left_output.green()
        } else {
            all_are_healthy = false;

            days_left_output.red()
        };
    let days_left_essential_output = 
        if days_until_end.num_days() as f64 <= calculated.days_left_essential {
            days_left_essential_output.green()
        } else {
            essential_are_healthy = false;

            days_left_essential_output.red()
        };

    println!(
        "...taking into account all expenses: {}",
        days_left_output,
    );
    println!(
        "...taking into account only essential expenses: {}",
        days_left_essential_output,
    );
    println!();

    if all_are_healthy {
        println!(
            "{}",
            "Your expenses are healthy, they should last you from your last \
            day on entry through your last day of the period.".green(),
        );
    } else {
        println!(
            "{}",
            "You are spending more than you can afford with your current \
            budget. Try minimizing your expenses".red(),
        );
        if essential_are_healthy {
            println!(
                "{}",
                "On the other hand, if you only spend money on essentials, \
                you should be able keep within your budget.".yellow(),
            );
        }
    }
}
