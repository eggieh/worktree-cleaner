# git-wt

Interactive Git worktree manager written in Rust.

`git-wt` discovers Git repositories inside configured directories, lists their worktrees, and lets you interactively remove them using proper Git commands.

It does **not** manually delete directories.

Internally it uses:

```bash
git worktree remove
```

so Git metadata stays consistent.

---

# Features

- Interactive worktree cleanup
- Multi-select deletion UI
- Safe deletion confirmation
- Dirty worktree detection
- Recursive repository discovery
- Proper Git worktree removal
- Configurable repository search roots
- Fast single-binary CLI
- Native Git subcommand support
- Diagnostic command (`git wt doctor`)

---

# Prerequisites

You must have:

- Git
- Rust toolchain

---

# Install Rust

Install Rust using rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Restart your shell afterward.

Verify installation:

```bash
rustc --version
cargo --version
```

---

# Installation

## Clone Repository

```bash
git clone https://github.com/eggieh/worktree-cleaner.git
cd worktree-cleaner
```

## Build

```bash
cargo build --release
```

Binary:

```bash
target/release/git-wt
```

---

# Global Installation

Install globally:

```bash
cargo install --path .
```

Verify:

```bash
git wt
```

Because the binary is named `git-wt`, Git automatically exposes it as:

```bash
git wt
```

This is the same mechanism used by tools like:

- git-lfs
- git-flow

---

# Configuration

Initialize config:

```bash
git wt init
```

This creates:

```bash
~/.config/git-wt/config.toml
```

Example:

```toml
roots = [
  "/dev",
  "/work"
]
```

`git-wt` recursively scans these directories for Git repositories.

---

# Usage

## Clean Worktrees

```bash
git wt clean
```

Example:

```text
api :: feature/auth
api :: spike/langgraph [DIRTY]
frontend :: fix/navbar
```

Controls:

- `SPACE` → select
- `ENTER` → continue
- confirmation prompt before deletion

Dirty worktrees are automatically skipped to prevent accidental data loss.

Example:

```text
Skipping dirty worktree (commit/stash changes first)
```

## Doctor

Run diagnostics:

```bash
git wt doctor
```

Example output:

```text
git-wt diagnostics

Config: ~/.config/git-wt/config.toml

Configured roots:
  ✓ /dev

Repositories found: 12

Repository summary:
  ✓ /dev/api (3 worktrees)
  ✓ /dev/frontend (1 worktrees)

Total worktrees: 4
```

Useful for:
- validating config
- verifying repository discovery
- debugging missing worktrees

---

# Creating Worktrees

If you do not already use Git worktrees, create one manually:

```bash
git worktree add ../my-feature-worktree feature/my-branch
```

Verify:

```bash
git worktree list
```

Example output:

```text
/dev/myrepo                  abc123 [main]
/dev/my-feature-worktree    def456 [feature/my-branch]
```

`git-wt clean` becomes useful once repositories contain additional worktrees.

---

# How Removal Works

`git-wt` uses:

```bash
git worktree remove --force <path>
```

This safely removes:

- worktree directory
- `.git/worktrees/*` metadata
- refs
- internal Git bookkeeping

---

# Troubleshooting

## `No git repos found`

Verify your configured roots:

```toml
roots = [
  "/dev"
]
```

Check manually:

```bash
find /dev -maxdepth 3 -name ".git"
```

---

## Repositories Found But No Worktrees Listed

Check existing worktrees:

```bash
git worktree list
```

If output only shows the main repository:

```text
/dev/myrepo  abc123 [main]
```

then no linked worktrees currently exist.

Create one:

```bash
git worktree add ../my-feature-worktree feature/my-branch
```

Verify again:

```bash
git worktree list
```

Example:

```text
/dev/myrepo                  abc123 [main]
/dev/my-feature-worktree    def456 [feature/my-branch]
```

---

## Dirty Worktrees Cannot Be Deleted

`git-wt` intentionally skips worktrees containing uncommitted changes.

Commit or stash changes first:

```bash
git stash
```

or:

```bash
git commit
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

# License

MIT License

Copyright (c) 2026 Ege Savci

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
