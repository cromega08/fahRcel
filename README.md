# fahRcel

A CLI tool to make Fahrenheit <--> Celsius convertions.

## Installation

```bash
$ cargo install fahrcel
```

## Requirements

* Rust > 2018 vr.

## Usage

To call fahRcel, type in your terminal:

```bash
$ fahrcel
```

The positional arguments required are:

* [output_type] option to convert numeric value:

	* f = 'to fahrenheit'
	* c = 'to celsius'

* [number] numeric value to convert

The final structure to use fahRcel is:

```bash
$ fahrcel [output_type] [number]
```

To get help, type:

```bash
$ fahrcel --help
```

## Examples

```bash
$ fahrcel c 5

Result: 5fh = -15.000001c
```

```bash
$ fahrcel c 5.6

Result: 5.6fh = -14.666667c
```

```bash
$ fahrcel c 5

Result: 5fh = -15.000001c
```

## Authors

* [@Cromega08](https://www.github.com/cromega08)

## License

* [GNU AGPL v3.0](https://choosealicense.com/licenses/agpl-3.0/)
