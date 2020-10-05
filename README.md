# finbudg

Quick cli tool to calculate your expenses and balance for a set period of time.

## TO-DO

* Take into account shared expenses
* Make AUR package
* Make error messages more useful
* Show what is being spent most money on
* (Maybe) a way to interactively edit an input file

## How to install

For now the only way to install this, is by cloning or downloading the repo, and
building it from source with cargo:

```
cargo build --release
```

From there, if you would like to have this program on your path, you can copy
it -- for example on Arch Linux -- to `/usr/bin/`.

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
  "products",
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
  category = "products"
  shared = 2

  [[days.expenses]]
  name = "Yoghurt"
  price = 1.24
  category = "products"
  qty = 2

  [[days.expenses]]
  name = "Onion"
  price = 0.15
  category = "products"

  [[days.expenses]]
  name = "Chicken"
  price = 2.28
  category = "products"
  shared = 2

[[days]]
date = 2020-10-02

  [[days.expenses]]
  name = "VPS"
  price = 5.0
  category = "utilities"
  recurring = true

  [[days.expenses]]
  name = "Transport card"
  price = 6.9
  category = "transport"
```

### Output:

```
Your expenses for the period of 2020-10-01 - 2020-10-31
Last day on entry: 2020-10-02
Days until period end: 29
Budget: 420.00

Average per day in utilities: 2.50
Average per day in supplies: 1.91
Average per day in transport: 3.45
Average per day in products: 3.50
Average per day in essential expenses: 9.45
Average per day: 11.36

Total in products: 7.00
Total in transport: 6.90
Total in supplies: 3.81
Total in utilities: 5.00
Total in essential expenses: 18.90
Total: 22.71

Left on balance: 397.29

Days until balance runs out:
..taking into account all expenses: 34.99
..taking into account only essential expenses: 42.04

Your expenses are healthy, they should last you from your last day on entry
through your last day of the period.
```
