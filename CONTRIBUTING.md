# Contributing

## Development Workflow

1. Create a GitHub issue or pick an existing one
2. Create a branch from `main` following the [branch naming](#branch-naming) convention
3. Make your changes
4. Run checks:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```
5. Commit following the [commit message](#commit-messages) convention
6. Open a pull request against `main`

## Branch Naming

Branches follow the Angular type prefix with the project issue number:

```
<type>/VAN-<issue-number>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

Examples:

```
feat/VAN-42
fix/VAN-15
docs/VAN-7
```

## Commit Messages

Follow the [Angular commit message convention](https://github.com/angular/angular/blob/main/contributing-docs/commit-message-guidelines.md):

```
<type>(<scope>): <subject>
```

- Type and subject in **lowercase**, no period at the end
- Scope is recommended
- Subject ≤ 100 characters
- Use the same type list as branch naming

Examples:

```
feat(music): add queue repeat mode
fix(play): handle empty playlist gracefully
docs(readme): update command list
```

## Tests

- Place unit tests in the same file using `#[cfg(test)] mod tests`
- Follow Arrange-Act-Assert pattern
- Use `test_` prefix for function names
- Prefer `assert_eq!` over `assert!`

## Gitignore Policy

`.gitignore` tracks only project artifacts (`/target/`, `.env`, etc.). Personal tooling configs (`.idea/`, `.vscode/`, `.DS_Store`) belong in your global gitignore (`~/.gitignore`), not in the repository.
