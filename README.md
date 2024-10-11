
# Threshold-re-encryption 

Secure communications and data storage are major challenges in the challenges in the field of cryptography. Algorithms based on the "Learning With Errors" (LWE) problem and Shamir's secret sharing have have emerged as robust solutions for ensuring information confidentiality and integrity. In this paper, we explore an approach that combines these two cryptographic techniques to create a threshold-based re-encryption system. Our main objective is to answer the following problem: “A company needs to secure its vault password. It could use a standard method but what if the key holder is unavailable or dies? What happens if the key is compromised by a malicious hacker or if the key holder decides to betray the company, and uses his power over the safe to his own advantage?“ To guarantee the security of this password, n colleagues in the company have access to what we call a “share”, a part of the password. It takes t + 1 shares to reconstruct this password, so that a number of shares lower than t does not allow access the password.

In the first two sections of the paper, we provide the mathematical basis for the implementation. These include proofs of t-privacy, as well as the formula for reconstruction from shares. In the next two sections, we look at the 0 and 1-privacy cases for their implementation, including definitions of the Encryption and Decryption functions, and the affine form corresponding to the machine system.

This project was created by Clemace Audibet, Gabrielle Lalou, and Alexis Mellier.

## Directory Structure

```
.
│   Threshold re-encryption.pdf
│   secret-sharing.pdf
│   SUIVI.md
│
│
├───code_intermédiaire
│   │   Cargo.lock
│   │   Cargo.toml
│   │
│   ├───src
│   │   └───bin
│   │           Gaussian.rs
│   │           Gaussian.rs.old
│   │           lwe.rs
│   │           secret_share.rs
│   │           testShare.rs
│   │
│   └───target
│           .rustc_info.json
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
            ├───old
            │       gaussian.rs
            │       Gaussian.rs.old
            │       lagrange_interpolation
            │       lwe.rs
            │       SSSS.rs
            │       test.rs
            │       test_gaussian.rs
            │       test_regev.rs

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
