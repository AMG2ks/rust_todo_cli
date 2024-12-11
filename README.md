# To-Do CLI

A simple Command-Line Interface (CLI) application to manage your to-do list, built with Rust.

This app allows you to:
- Add tasks to a to-do list.
- List all tasks.
- Remove tasks by their ID.

---

## Features
- Built using Rust for high performance and reliability.
- Tasks are stored in a `tasks.json` file for persistence.
- Simple and intuitive CLI commands.

---

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/todo-cli.git
   cd todo-cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run -- [COMMAND]
   ```

---

## Usage

### Add a Task
```bash
cargo run -- add "Task description"
```

Example:
```bash
cargo run -- add "Learn Rust"
```

### List Tasks
```bash
cargo run -- list
```

### Remove a Task
```bash
cargo run -- remove <TASK_ID>
```

Example:
```bash
cargo run -- remove 1
```

---

## Requirements
- [Rust](https://www.rust-lang.org/tools/install) (version 1.70 or later)

---

## Project Structure

```
.
├── src
│   ├── main.rs     # Main application logic
├── tasks.json       # JSON file to store tasks
├── Cargo.toml       # Rust package configuration
└── README.md        # Documentation
```

---

## Contributing
Contributions are welcome! Feel free to fork the repository and submit a pull request.

---

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Author
Aziz Guebsi