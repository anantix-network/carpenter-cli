# Contributing to Carpet CLI

👋 First off, thanks for taking the time to contribute!

## 🚀 Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/anantix-network/carpenter-cli
   cd carpenter-cli
   ```
3. Create a new branch:
   ```bash
   git checkout -b feature/amazing-feature
   ```
4. Make your changes
5. Run tests:
   ```bash
   cargo test --features test-utils
   ```

## 💻 Development Setup

### Prerequisites

- Rust (latest stable)
- Cargo

### Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test --features test-utils

# Run specific test
cargo test test_name --features test-utils
```

## 📝 Coding Guidelines

### Rust Style Guide

- Follow [Rust Style Guide](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code
- Use `clippy` for linting
- Add documentation comments for public APIs

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and pull requests liberally after the first line

Example:
```
feat: Add support for environment variables in scripts

- Add env parsing in config loader
- Add env merging in script runner
- Update documentation
- Add tests for env functionality

Fixes #123
```

### Pull Request Process

1. Update the README.md with details of changes if needed
2. Add/update tests for any new functionality
3. Make sure all tests pass
4. Update documentation if needed
5. The PR will be merged once you have the sign-off of two maintainers

## 🔍 Code Review Process

1. Each PR needs at least two approvals from maintainers
2. Changes may be requested if:
   - Tests are failing
   - Code style doesn't match guidelines
   - Missing tests or documentation
   - Implementation concerns

## 🐛 Bug Reports

When filing an issue, make sure to answer these questions:

1. What version of Carpet CLI are you using?
2. What operating system are you using?
3. What did you do?
4. What did you expect to see?
5. What did you see instead?

## 💡 Feature Requests

Feature requests are welcome! Please provide:

1. Clear use case
2. Expected behavior
3. Why this would be useful for other users

## 📦 Adding Dependencies

Before adding a new dependency, consider:

1. Is it really necessary?
2. Is it actively maintained?
3. License compatibility
4. Impact on build time and binary size

## 🧪 Testing Guidelines

### Unit Tests

- Place unit tests in the same file as the code
- Use descriptive test names
- Test edge cases
- Add documentation for complex test scenarios

### Integration Tests

- Place integration tests in `tests/` directory
- Use meaningful test file names
- Test real-world usage scenarios

## 📖 Documentation

- Update README.md for user-facing changes
- Add rustdoc comments for public APIs
- Include examples in documentation
- Keep CHANGELOG.md up to date

## 🛠 Tools We Use

- `cargo fmt` for code formatting
- `cargo clippy` for linting
- `cargo test` for testing
- `cargo doc` for documentation

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.
