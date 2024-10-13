
# Threshold-re-encryption 



## Directory Structure

```
.
│   Threshold re-encryption.pdf
│   secret-sharing.pdf
│   SUIVI.md
│
│
└───code
    │   Cargo.lock
    │   Cargo.toml
    │
    └───src
        └───bin
            │   EoD.rs
            │   EoD_sys_initial.rs
            │   from_file.rs
            │   lwe_functions.rs
            │   secret_sharing.rs
            │
            ├───keys
            │       cleAlice.txt
            │       cleBob.txt
            │       machine1
            │       randomness
            │       r_secret_share
            │       skA_secret_share
            │

```

## How to Run the Project

To execute a specific file from the project, follow these steps:

1. Navigate to the `code_projet/src/bin/` directory:

    ```bash
    cd code_projet/src/bin/
    ```

2. Run the desired file using Cargo:

    ```bash
    cargo run --bin <filename>
    ```

    Replace `<filename>` with the name of the file you want to execute (e.g., `secret_sharing.rs`).

## Setting Up Rust Environment

To set up the Rust environment for this project, follow the steps below:

### 1. Install Rust

If you don't have Rust installed, you can install it using the official installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. Once installed, add Cargo's bin directory to your `PATH` environment variable.

To verify the installation, run:

```bash
rustc --version
```

### 2. Clone the Repository

Clone this repository to your local machine:

```bash
git clone https://github.com/Alexisdevpro/Threshold-re-encryption.git
cd Threshold-re-encryption
```

### 3. Build the Project

Run the following command to build the project:

```bash
cargo build
```

This will compile the Rust code and generate necessary binaries in the `target/` directory.

### 4. Run Tests (Optional)

To ensure everything is working correctly, run the test suite using:

```bash
cargo test
```

### 5. Docker Support

Coming soon... Next feature 

## Documentation

- **Bibliography**: The `Bibliographie/` directory contains research articles and other reference materials used for this project.
- **Project Reports**: Project reports and initial research can be found in the root directory (`PROJ104_latex_24.06.pdf`, `secret-sharing.pdf`, etc.).
