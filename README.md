# finbudg

Quick cli tool to calculate your expenses and balance for a set period of time.

## TO-DO

* Make error messages more useful
* Show what is being spent most money on
* (Maybe) a way to interactively edit an input file

## How to install

You need to install the Rust toolchain (rustup, cargo, etc) to build this
program. You will also need scdoc to generate the manual pages, and make if you
want to automatically build and install the program.

You can build it yourself using cargo:

```
cargo build --release
```

Or build it and install it using make:

```
make
sudo make install
```

You can remove it by executing:

```
sudo make uninstall
```

For more information about the usage of this program, see `man 1 finbudg` and
`man 5 finbudg` after building and installing with make.

## Example

```
finbudg input.toml
```

### Input:

```toml
start_date = 2020-10-01
end_date = 2020-10-31
budget = 420.0
essential_categories = [
  "produce",
  "transport",
  "utilities",
]

[[days]]
date = 2020-10-01

  [[days.expenses]]
  name = "Potato masher"
  price = 3.81
  category = "supplies"

  [[days.expenses]]
  name = "Bacon"
  price = 3.33
  category = "produce"
  shared = ["Fox", "Falco"]

  [[days.expenses]]
  name = "Yoghurt"
  price = 1.24
  category = "produce"
  owed = true
  shared = ["Falco"]

  [[days.expenses]]
  name = "Onion"
  price = 0.15
  category = "produce"

  [[days.expenses]]
  name = "Chicken"
  price = 2.28
  category = "produce"
  shared = ["Fox"]

[[days]]
date = 2020-10-04

[[days]]
date = 2020-10-02

  [[days.expenses]]
  name = "VPS"
  price = 5.0
  category = "utilities"

  [[days.expenses]]
  name = "Transport card"
  price = 6.9
  category = "transport"

```

### Output:

```
Your expenses for the period of 2020-10-01 - 2020-10-31
Last day on entry: 2020-10-04
Days until period end: 27
Budget: 420.00

Average per day in produce: 0.60
Average per day in supplies: 0.95
Average per day in utilities: 1.25
Average per day in transport: 1.73
Average per day in essential expenses: 3.58
Average per day: 4.53

Total in produce: 2.40
Total in utilities: 5.00
Total in supplies: 3.81
Total in transport: 6.90
Total in essential expenses: 14.30
Total: 18.11

Left on balance: 401.89

Fox owes you in shared expenses: 2.25
Falco owes you in shared expenses: 2.35
In total you're owed: 4.60
Assuming you haven't been repaid, you're left with: 397.29

Days until balance runs out:
...taking into account all expenses: 88.77
...taking into account only essential expenses: 112.42

Your expenses are healthy, they should last you from your last day on entry through your last day of the period.
```
