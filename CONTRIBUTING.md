# Contributing

## Prerequisites

- Rust 2024 edition
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed via pip (not apt):
  ```bash
  pip3 install "yt-dlp[default] @ https://github.com/yt-dlp/yt-dlp/archive/master.tar.gz"
  ```
- [Deno](https://deno.land) in PATH — required by yt-dlp for YouTube JS challenges
- Discord bot token with voice permissions

## Setup

```bash
git clone https://github.com/kremeshnoi/turtle-van.git
cd turtle-van
cp .env.example .env
# Add your DISCORD_TOKEN to .env
cargo run
```

## Development Workflow

1. Create a branch from `main`
2. Make your changes
3. Run checks before committing:
   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test
   ```
4. Commit using [Angular convention](#commit-messages)
5. Open a pull request against `main`

## Commit Messages

Follow the Angular (Google) format:

```
<type>(<scope>): <subject>
```

- Type and subject in **lowercase**, no period at the end
- Scope is recommended
- Subject ≤ 100 characters

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`

Examples:
```
feat(music): add queue repeat mode
fix(play): handle empty playlist gracefully
docs(readme): update command list
```

## Git Ignore Policy

`.gitignore` in this repository only tracks project artifacts (`/target/`, `.env`, etc.). Personal tooling configs (IDE settings, editor configs like `.idea/`, `.vscode/`, `.DS_Store`) must be added to your global gitignore (`~/.gitignore`), not to the repository's `.gitignore`.

## Code Style

- 4-space indentation, ~100 char line length
- No comments unless the logic is non-obvious
- `?` operator for error propagation
- Domain errors via `thiserror`

## Adding a New Command

1. Create a file in `src/features/music/youtube/commands/`
2. Define the command with `#[poise::command(prefix_command, slash_command)]`
3. Add it to the vec in `commands/mod.rs`

## Tests

- Place unit tests in the same file using `#[cfg(test)] mod tests`
- Follow Arrange-Act-Assert pattern
- Use `test_` prefix for function names
- Prefer `assert_eq!` over `assert!`
