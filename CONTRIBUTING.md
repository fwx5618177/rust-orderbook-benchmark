# Contributing Guidelines

Thank you for your interest in contributing to the Orderbook project! This document provides guidelines and steps for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## How to Contribute

1. **Fork the Repository**
   - Fork the repository to your GitHub account
   - Clone your fork locally

2. **Create a Branch**
   - Create a branch for your changes
   - Use a clear and descriptive branch name (e.g., `feature/add-new-tree-implementation` or `fix/memory-leak-in-bptree`)

3. **Make Your Changes**
   - Write clear, commented code
   - Follow the existing code style and conventions
   - Add tests for new features
   - Update documentation as needed

4. **Test Your Changes**
   - Run the existing test suite
   - Add new tests for your changes
   - Ensure all tests pass

5. **Submit a Pull Request**
   - Push your changes to your fork
   - Create a pull request from your branch to our main branch
   - Provide a clear description of the changes
   - Link any relevant issues

## Development Setup

```bash
# Clone the repository
git clone https://github.com/fwx5618177/rust-orderbook-benchmark.git
cd rust-orderbook-benchmark

# Install dependencies
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Coding Standards

- Follow Rust standard coding conventions
- Use meaningful variable and function names
- Comment complex algorithms and implementations
- Keep functions focused and concise
- Write comprehensive tests

## Commit Messages

- Use clear and meaningful commit messages
- Start with a verb in the present tense
- Keep the first line under 50 characters
- Provide detailed description if needed

Example:
```
Add B+Tree bulk insertion optimization

- Implement batch insertion for better performance
- Add tests for bulk operations
- Update documentation with new feature
```

## Questions or Problems?

Feel free to:
- Open an issue for discussion
- Ask questions in pull requests
- Contact the maintainers directly

## License

By contributing, you agree that your contributions will be licensed under the MIT License. 