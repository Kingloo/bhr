# bhr

Command line to turn a raw number of bytes into a human-readable SI file size.

Accepted units are:

	`k` for kebibytes

	`m` for mebibytes

	`g` for gibibytes

	`t` for tebibytes

	`p` for pebibytes

	`e` for exbibytes

## Chooses unit automatically

`bhr 1000000000`

	953.67 MiB

## With specific unit

`bhr k 1000000000`

	976562.50 KiB

## Reading from stdin

echo 1000000000 | bhr

Or

echo 1000000000 | bhr -k

## Limitations

Limited to the maximum value of a signed 64-bit integer, aka `9223372036854775807`, or about 8 exbibytes.