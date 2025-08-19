# Rust Guessing Game

A simple command-line number guessing game written in Rust. The program picks a random number between 1 and 100 and prompts you to guess until you get it right, guiding you with "Too small!" or "Too big!" messages.

## Features
- Random secret number in the range 1..=100
- Input validation with friendly error messages
- Instant feedback on each guess

## Tech Stack
- Rust 1.70+ (edition 2021)
- Crate: `rand = "0.8.5"`

## Project Structure
```
rust/
├─ Cargo.toml        # Package metadata and dependencies
├─ Cargo.lock        # Locked dependency versions (committed)
├─ src/
│  └─ main.rs        # Game entrypoint and logic
└─ README.md
```

## Getting Started
### Prerequisites
- Install Rust toolchain (rustup): https://rustup.rs/
- On Windows, use PowerShell or Command Prompt. Ensure `%USERPROFILE%\.cargo\bin` is on your PATH.

### Install dependencies
Dependencies are managed by Cargo. No manual steps are needed—`cargo build` will fetch them automatically.

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```
Example session:
```
Enter your guess:
50
Too big!
Enter your guess:
23
Too small!
Enter your guess:
37
You win!
```

## How It Works
Core logic lives in `src/main.rs`:
- Generates `secret_num` with `rand::thread_rng().gen_range(1..=100)`
- Reads user input line-by-line, validates via `str::parse::<u8>()`
- Compares with `std::cmp::Ordering` to print hints until correct

## Development
- Format code: `cargo fmt`
- Lint (if you have Clippy): `cargo clippy -- -D warnings`
- Run tests (none yet): `cargo test`

## Notes
- The `.gitignore` excludes build artifacts (e.g., `target/`) and editor/OS junk. `Cargo.lock` is kept under version control.

## License
Add a license of your choice (e.g., MIT/Apache-2.0) if you plan to share this publicly.
