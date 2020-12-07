finbudg(1)

# NAME

finbudg - Quick cli tool to calculate your expenses and balance for a set period
of time.

# SYNOPSIS

*finbudg* [FLAGS] <_file_>

# DESCRIPTION

This program helps you calculate your expenses and balance by providing it with
a file containing a list of expenses made for a specified period of time. See
*finbudg*(5) for more information on the syntax of this file.

# FLAGS

*-h, --help*
	Prints the list of flags available.

*-w, --consider-owed*
	Take into account what's owed when calculating the total
	and subtotals.

*-p, --plain*
	Don't colorize the output. Can also be set with the *NO_COLOR* environment
	variable.

*--force-color*
	Forces colorized output even when piping. Takes precedence over *--plain*
	flag and *NO_COLOR* environment variable.

*-V, --version*
	Prints version information and exits.

# SEE ALSO

*finbudg*(5)

# AUTHORS

Yaroslav de la Peña Smirnov <yps@yaroslavps.com>.
