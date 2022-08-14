# french-numbers

Convert numbers into French words. Maximum supported figure is 999,999.

## Installation

The latest [release](https://github.com/neilbryson/reddownload/releases) binaries can be downloaded here

- [Linux](https://github.com/neilbryson/french-numbers/releases/latest/download/french-numbers-linux.tar.gz)
- [macOS](https://github.com/neilbryson/french-numbers/releases/latest/download/french-numbers-macOS.tar.gz)
- [Windows](https://github.com/neilbryson/french-numbers/releases/latest/download/french-numbers-windows.zip)

## Usage

Run the binary in the command line and pass a space-separated list of numbers

```bash
./french-numbers 0 1 3 100 92398 10288

0 => "zéro"
1 => "un"
3 => "trois"
99 => "quatre-vingt-dix-neuf"
162 => "cent-soixante-deux"
1000 => "mille"
92398 => "quatre-vingt-douze-mille-trois-cent-quatre-vingt-dix-huit"
102888 => "cent-deux-mille-huit-cent-quatre-vingt-huit"
```


#### Dataset

```bash
./french-numbers 0 1 5 10 11 15 20 21 30 35 50 51 68 70 75 99 100 101 105 111 123 168 171 175 199 200 201 555 999 1000 1001 1111 1199 1234 1999 2000 2001 2020 2021 2345 9999 10000 11111 12345 123456 654321 999999
```

## Building from source

### Requirements
- [rustup](https://rustup.rs/)

### Debug build

To run the debug build, run `cargo run -- [ARGS]`

```bash
cargo run -- 4 20 80
```

### Release binary

```bash
cargo build --release
```

### Running the program

Simply execute the binary at `target/release` and pass an array of numbers.

## License
MIT
