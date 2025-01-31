# RustCrackMaster

RustCrackMaster is an educational CrackMe project written in Rust, designed to enhance skills in reverse engineering, code analysis, and software security. The program verifies user-entered activation passwords using basic obfuscation techniques to complicate code analysis.

## Features

- Simple command-line interface
- Activation password verification with basic obfuscation
- Protection against straightforward code analysis
- Detailed documentation for beginners

## Installation

### Prerequisites

- Rust (version 1.50 or above)
- Cargo (comes with Rust)

### Steps

1. **Clone the repository:**
    ```bash
    git clone https://github.com/yourusername/RustCrackMaster.git
    cd RustCrackMaster
    ```

2. **Build the project:**
    ```bash
    cargo build --release
    ```

    The compiled binary will be located in `target/release/`.

## Usage

Run the program using the following command:

```bash
./target/release/RustCrackMaster
