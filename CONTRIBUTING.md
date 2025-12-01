# Contributing to HoneyTrap

Thank you for your interest in contributing to HoneyTrap! 🎉

## Code of Conduct

Be respectful, inclusive, and collaborative.

## How to Contribute

### 1. Set up your development environment

```bash
# Clone the repository
git clone https://github.com/yourusername/honeytrap.git
cd honeytrap

# Run setup script
./scripts/setup-dev.sh

# Or manually
make deps
make build
```

### 2. Create a branch

```bash
git checkout -b feature/my-amazing-feature
```

### 3. Make your changes

- Write clean, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 4. Test your changes

```bash
# Run all checks
make ci

# Or individually
make fmt        # Format code
make lint       # Run clippy
make test       # Run tests
```

### 5. Commit your changes

```bash
git add .
git commit -m "feat: add amazing feature"
```

**Commit message format:**

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

### 6. Push and create a Pull Request

```bash
git push origin feature/my-amazing-feature
```

Then create a Pull Request on GitHub.

## Development Guidelines

### Code Style

- Use `rustfmt` for formatting: `make fmt`
- Follow Rust naming conventions
- Write self-documenting code with clear variable names
- Add comments for complex logic

### Testing

- Write unit tests for all new functions
- Add integration tests for new features
- Aim for >80% code coverage
- Test edge cases and error conditions

### Documentation

- Add doc comments (`///`) for public APIs
- Update README.md for user-facing changes
- Add examples in doc comments
- Keep CHANGELOG.md updated

### Performance

- Use `cargo bench` for performance-critical code
- Avoid unnecessary allocations
- Profile before optimizing
- Document performance considerations

## Pull Request Process

1. Ensure all tests pass: `make ci`
2. Update documentation
3. Add entry to CHANGELOG.md
4. Request review from maintainers
5. Address review feedback
6. Wait for approval and merge

## Questions?

- Open an issue for questions
- Join discussions in GitHub Discussions
- Check existing issues and PRs first

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
