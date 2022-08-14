# french-numbers

Convert numbers into French words

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

## Building from source

### Requirements
- [rustup](https://rustup.rs/)

### Release binary

```bash
cargo build --release
```

### Running the program

Simply execute the binary at `target/release` and pass an array of numbers.

## License
MIT
