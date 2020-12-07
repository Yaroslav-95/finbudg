finbudg(5)

# NAME

finbudg - Quick cli tool to calculate your expenses and balance for a set period
of time.

# DESCRIPTION

This manual page documents the syntax of the expenses files used by the
*finbudg* program.

# SYNTAX

Expenses files used by *finbudg* use the toml file format as their base for the
syntax.

## PREAMBLE

The expenses file should contain at the start the following keys:

*start_date*=YYYY-MM-DD
	This is the date when the period for this budget starts, e.g. your payday.

*end_date*=YYYY-MM-DD
	This is the date when the period for this budget ends, e.g. the day before
	your next payday.

*budget*=float64
	A floating point number that represents your budget for the specified period
	of time.

*essential_categories*=String[]
	An array of strings representing the categories that you want to take into
	account as "essential". _Optional_.

*[[days]]*
	An entry of this type is an entry for the array of tables *days* and
	contains the following keys:

		*date*=YYYY-MM-DD
			The date for this day when the following expenses were made.

		*[[days.expenses]]*
			An entry of this type is an entry for the nested array of tables
			*expenses* inside of an entry of type *days*. It may contain the
			following keys:

				*name*=String
					The name of the product/service/etc. that money was spent
					on.

				*price*=float64
					How much money was spent on said product/service/etc.

				*category*=String
					The category to which this expense belongs. _Optional_.

				*shared*=array<String>
					An list of names of the people that this expense is shared
					with proportionally. If owed equals true, then you are not
					part of this expense, and the people listed here owe you all
					of this expense. _Optional_.

				*owed*=boolean
					A boolean value that indicates whether you paid for this for
					another person or persons. If set to true, then shared
					should contain at least one name. _Optional_.


# NOTES

The dates should be in ISO 8601 format with *NO* time in them, e.g.
*2020-04-20*.

You don't have to include every day in the period in your expenses file if you
didn't spend any money that day. The program will take into account the last
date in the file to count the number of days. Also, the *[[days.expenses]]* key
is optional, you can also include a day and omit the expenses.

If you're not very familiar with the TOML format, you should take a look at
*https://toml.io/en/v1.0.0-rc.2*. Especially the part about arrays of tables.

If you're still not sure how the input file should look like, or if you prefer
to learn by example, take a look at *https://github.com/Yaroslav-95/finbudg*.
The README.md includes an example of an expense file.

# SEE ALSO

*finbudg*(1)

# AUTHORS

Yaroslav de la Peña Smirnov <yps@yaroslavps.com>.
