<h1 align="center">
    The Aura compiler (<code>aurac</code>) 
</h1>

<div  align="center">
    <img src="https://img.shields.io/github/stars/auralangco/aurac" alt="GitHub stars" href="https://github.com/auralangco/aurac/stargazers" />
    <img src="https://img.shields.io/github/release/auralangco/aurac" alt="GitHub release" href="https://github.com/auralangco/aurac/releases" />
    <img src="https://img.shields.io/github/license/auralangco/aurac" alt="License" href="https://raw.githubusercontent.com/auralangco/aurac/master/LICENSE" />
</div>

<p align="center" style="text-align: justify;">
    <code>aurac</code> is the official compiler for the <strong>Aura programming language</strong>, a functional, parallelizable, and multitarget language. The current version targets the C programming language, with plans to extend support to JavaScript and other languages in the future.
</p>

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Example Code](#example-code)
- [Future Plans](#future-plans)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Functional Language**: Emphasizes immutability and functional programming paradigms.
- **Parallelizable**: Designed to run efficiently on multicore processors.
- **Multitarget**: Initially targets C, with plans for JavaScript and more.
- **Type Safety**: Ensures operations are validated during compile time.
- **Compile-Time and Runtime Operations**: Allows compile-time operations to be available at runtime while enforcing proper compile-time treatment.

## Getting Started

To get started with `aurac`, clone the repository and follow the instructions below to set up and use the compiler.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (required to build the compiler)

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (required to build the compiler)

- A `C` compiler (required to compile the generated C code)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/auralangco/aurac.git
   cd aurac
   ```

2. Build the project:
   ```sh
   cargo build
   ```

## Usage

To compile an Aura program, use the following command:

```sh
cargo run -- path/to/yourfile.aura
```

This will generate a C file from your Aura code, which can then be compiled using a C compiler like `gcc`.

## Example Code

Here is a simple example of Aura code from the [Aura repository](https://github.com/auralangco/aura/blob/master/examples/core.aura):

`Function to add two numbers`
```aura
fn add(a: int, b: int): int {
    return a + b;
}
```

`Main function`
```aura
fn main() {
    let result = add(5, 7);
    println("The result is {}", result);
}
```

This example demonstrates a basic function definition and usage in Aura, showcasing the language's syntax and functional programming style.

## Future Plans

- Front-End Development: Create a frontend to compile Aura code properly into C.

- Compiler Wrapper: Develop a wrapper for gcc/clang/mingw to compile the generated C code into an executable.

- C Interoperability: Facilitate easy interoperability with C native code.

- Additional Targets: Add support for JavaScript and C for Windows.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request. Here’s how you can help:

1. Fork the repository.

   ```sh
   git clone https://github.com/auralangco/aurac.git
   ```

2. Create a new branch.

   ```sh
   git checkout -b feature-branch
   ```

3. Make your changes and commit them.

   ```sh
   git commit -am 'Add new feature'
   ```

4. Push to the branch.

   ```sh
   git push origin feature-branch
   ```

5. Create a new Pull Request.

- _You can also contribute by opening an issue to report a bug or request a new feature._

## License

This project is licensed under the [`Apache-2.0 License`](https://www.apache.org/licenses/LICENSE-2.0.txt). See the [LICENSE](LICENSE) file for more details.

## Contact

- [**Aura GitHub**](https://github.com/auralangco/aura)
- [**Aura Compiler GitHub**](https://github.com/auralangco/aurac)
- [**Aura Lang Discord**](https://discord.gg/avhRgaTRUt)
