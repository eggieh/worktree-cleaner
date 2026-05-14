# worktree-cleaner
Simple blazingly fast worktree cleaner for devs
# wt

Interactive Git worktree cleaner written in Rust.

`wt` scans configured directories, discovers Git repositories, lists their worktrees, and lets you interactively remove them using Git itself.

It does **not** just delete folders.

Internally it runs:

```bash
git worktree remove
```

so Git metadata is cleaned properly.

---

# Features

- Interactive worktree selection
- Multi-select deletion UI
- Automatically discovers repositories
- Uses proper Git worktree removal
- Configurable root directories
- Single static binary
- Fast startup

---

# Prerequisites

You must have:

- Git
- Rust toolchain

## Install Git

### Ubuntu

```bash
sudo apt install git
```

### macOS

```bash
brew install git
```

---

# Install Rust

Install Rust using rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then restart your shell and verify:

```bash
rustc --version
cargo --version
```

---

# Installation

## Clone Repository

```bash
git clone https://github.com/YOUR_USERNAME/wt.git
cd wt
```

## Build

```bash
cargo build --release
```

Binary will be located at:

```bash
target/release/wt
```

---

# Global Installation

Install globally using Cargo:

```bash
cargo install --path .
```

Then verify:

```bash
wt --help
```

---

# Configuration

Initialize config:

```bash
wt init
```

This creates:

```bash
~/.config/wt/config.toml
```

Example config:

```toml
roots = [
  "/dev",
  "/work"
]
```

`wt` will recursively search these directories for Git repositories.

---

# Usage

## Clean Worktrees

```bash
wt clean
```

Example:

```text
[api-server] feature/auth-redesign /dev/api-server-auth
[frontend] fix/navbar-overflow /dev/frontend-fix
```

Use:

- `SPACE` → select
- `ENTER` → confirm deletion

---

# How Worktree Removal Works

`wt` uses:

```bash
git worktree remove --force <path>
```

This safely removes:

- worktree directory
- `.git/worktrees/*` metadata
- references
- internal Git bookkeeping

---

# Project Structure

```text
src/
 └── main.rs
```

---

# Development

Run locally:

```bash
cargo run -- clean
```

Format:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy
```

---

# Future Ideas

- fuzzy search
- branch age display
- merged branch cleanup
- stale worktree detection
- TUI mode
- GitHub PR integration
- `wt new`
- `wt open`

---

# License

MIT
