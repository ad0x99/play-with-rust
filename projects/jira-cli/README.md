# Jira CLI - Introduction

Jira CLI is a command-line interface (CLI) tool designed to help users manage their Jira tasks efficiently. This tool allows users to navigate through different pages, handle various actions, and interact with a database to manage epics and stories.

# Project Structure

The project is organized as follows:

```rust
Cargo.lock
Cargo.toml
data/
    db.json
README.md
src/
    db.rs
    io_utils.rs
    main.rs
    models.rs
    navigators.rs
    ui/
        mod.rs
        pages/
            mod.rs
            page_helpers.rs
        prompts.rs
target/
```

## Key Files and Directories

- **Cargo.toml**: Contains the project dependencies and metadata.
- **data/db.json**: The JSON file used to store the database state.
- **src/**: The source code directory.
  - **db.rs**: Contains the database-related functionality.
  - **io_utils.rs**: Utility functions for input/output operations.
  - **main.rs**: The main entry point of the application.
  - **models.rs**: Defines the data models used in the project.
  - **navigators.rs**: Handles navigation between different pages.
  - **ui/**: Contains the user interface components.
  - **mod.rs**: The main module for the UI components.
  - **pages/**: Contains the page-specific UI components.
  - **mod.rs**: The main module for the pages.
  - **page_helpers.rs**: Helper functions for the pages.
  - **prompts.rs**: Handles user prompts.

## Features

### Database Operations

The database operations are handled by the `JiraDatabase` struct, which provides methods to `create`, `read`, `update`, and `delete` _epics_ and _stories_. The database state is stored in a JSON file (`data/db.json`).

### Navigation

The navigation between different pages is managed by the `Navigator` struct. It maintains a stack of pages and handles actions such as navigating to epic details, story details, and creating or updating epics and stories.

### User Interface

The user interface components are defined in the ui module. The main pages include:

- **HomePage**: Displays a list of epics and allows navigation to epic details or creation of new epics.
- **EpicDetail**: Displays details of a specific epic and allows navigation to story details or creation of new stories.
- **StoryDetail**: Displays details of a specific story.

## Testing

The project includes unit tests for various components, ensuring the correctness of the database operations, navigation, and user interface interactions. The tests are located within the respective modules, such as `mod.rs` and `navigators.rs`.

## Dependencies

| Dependency                                          | Version | Purpose                                                                                                                                                                                                                   |
| --------------------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [anyhow](https://crates.io/crates/anyhow)           | 1.0     | Provides a simple error handling library for Rust. It allows you to easily create and manage errors with context, making debugging and error reporting more straightforward.                                              |
| [serde](https://crates.io/crates/serde)             | 1.0.217 | A powerful framework for serializing and deserializing Rust data structures. The derive feature enables automatic generation of serialization and deserialization code using macros.                                      |
| [serde_json](https://crates.io/crates/serde_json)   | 1.0     | A library for working with JSON data in Rust. It leverages serde to provide easy serialization and deserialization of JSON data.                                                                                          |
| [ellipse](https://crates.io/crates/ellipse)         | 0.2.0   | A library for text truncation with ellipsis. It helps in shortening strings and appending an ellipsis (...) to indicate that the text has been truncated.                                                                 |
| [itertools](https://crates.io/crates/itertools)     | 0.14.0  | Provides extra iterator adaptors, functions, and macros that are not included in the standard library. It enhances the functionality of iterators in Rust, making it easier to perform complex operations on collections. |
| [clearscreen](https://crates.io/crates/clearscreen) | 4.0.0   | A library to clear the terminal screen. It provides a cross-platform way to clear the console, which can be useful for creating command-line applications with a clean user interface.                                    |

# Getting Started

Prerequisites: Rust programming language installed on your machine.

Building the Project

- To build the project, run the following command:

```bash
cargo build
```

Running the Project

- To run the project, use the following command:

```bash
cargo run
```

Running Tests

- To run the tests, use the following command:

```bash
cargo test
```
