# rlox - A Lox Interpreter Written in Rust

`rlox` is a simple yet powerful interpreter for the Lox programming language, written in Rust. This project follows the concepts introduced in the *Crafting Interpreters* book by Robert Nystrom and is aimed at providing a performant and easy-to-use tool for running Lox code.

## Features

- Executes Lox programs from files.
- Written in Rust for high performance and safety.
- CLI interface for easy use.

## Installation

To install `rlox`, you will need to have Rust and Cargo installed on your system.

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rlox.git
   ```

2. Navigate to the project directory:
   ```bash
   cd rlox
   ```

3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

4. Optionally, you can install the binary globally:
   ```bash
   cargo install --path .
   ```

## Usage

Once installed, you can run Lox programs using the following command:

```bash
rlox [OPTIONS]
```

### Options:

- `--path <PATH>`: Specifies the path to the Lox file that you want to execute.
- `-h, --help`: Displays usage information and the available options.
- `-V, --version`: Displays the current version of the interpreter.

### Examples:

Running a Lox file:

```bash
rlox --path examples/hello_world.lox
```

Displaying help:

```bash
rlox --help
```

Displaying the version:

```bash
rlox --version
```

## Contributing

Contributions are welcome! If you'd like to contribute to `rlox`, feel free to open an issue or submit a pull request.
