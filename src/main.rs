// src/main.rs

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "wt")]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Clean,
    Init,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    roots: Vec<String>,
}

#[derive(Debug, Clone)]
struct Worktree {
    repo: String,
    path: PathBuf,
    branch: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Clean => clean_worktrees(),
    }
}

fn init_config() -> Result<()> {
    let path = config_path();

    if path.exists() {
        println!("{}", "Config already exists".yellow());
        return Ok(());
    }

    let parent = path.parent().unwrap();
    fs::create_dir_all(parent)?;

    let config = Config {
        roots: vec!["/dev".into()],
    };

    fs::write(&path, toml::to_string_pretty(&config)?)?;

    println!(
        "{} {}",
        "Created config:".green(),
        path.display()
    );

    Ok(())
}

fn clean_worktrees() -> Result<()> {
    let config = load_config()?;

    let repos = discover_git_repos(&config.roots)?;

    if repos.is_empty() {
        println!("{}", "No git repos found".yellow());
        return Ok(());
    }

    let mut all_worktrees = vec![];

    for repo in repos {
        let worktrees = get_worktrees(&repo)?;

        for wt in worktrees {
            all_worktrees.push(wt);
        }
    }

    if all_worktrees.is_empty() {
        println!("{}", "No removable worktrees found".yellow());
        return Ok(());
    }

    let items: Vec<String> = all_worktrees
        .iter()
        .map(|wt| {
            format!(
                "[{}] {} {}",
                wt.repo.cyan(),
                wt.branch
                    .clone()
                    .unwrap_or_else(|| "detached".into())
                    .yellow(),
                wt.path.display()
            )
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select worktrees to delete")
        .items(&items)
        .interact()?;

    if selections.is_empty() {
        println!("{}", "Nothing selected".yellow());
        return Ok(());
    }

    for idx in selections {
        let wt = &all_worktrees[idx];

        println!(
            "{} {}",
            "Removing".red(),
            wt.path.display()
        );

        remove_worktree(wt)?;
    }

    Ok(())
}

fn load_config() -> Result<Config> {
    let path = config_path();

    if !path.exists() {
        return Err(anyhow!(
            "Config not found. Run: wt init"
        ));
    }

    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

fn config_path() -> PathBuf {
    config_dir()
        .unwrap()
        .join("wt")
        .join("config.toml")
}

fn discover_git_repos(roots: &[String]) -> Result<Vec<PathBuf>> {
    let mut repos = vec![];

    for root in roots {
        for entry in WalkDir::new(root)
            .max_depth(3)
            .follow_links(false)
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().is_dir() {
                continue;
            }

            let git_dir = entry.path().join(".git");

            if git_dir.exists() {
                repos.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(repos)
}

fn get_worktrees(repo: &Path) -> Result<Vec<Worktree>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()
        .with_context(|| {
            format!("Failed to inspect {}", repo.display())
        })?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    parse_worktrees(
        repo.file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        &stdout,
    )
}

fn parse_worktrees(
    repo_name: String,
    input: &str,
) -> Result<Vec<Worktree>> {
    let mut result = vec![];

    let blocks = input.split("\n\n");

    for block in blocks {
        if block.trim().is_empty() {
            continue;
        }

        let mut path = None;
        let mut branch = None;
        for line in block.lines() {
            if let Some(v) = line.strip_prefix("worktree ") {
                path = Some(PathBuf::from(v));
            }

            if let Some(v) = line.strip_prefix("branch refs/heads/") {
                branch = Some(v.to_string());
            }
        }

        let path = match path {
            Some(p) => p,
            None => continue,
        };

        result.push(Worktree {
            repo: repo_name.clone(),
            path,
            branch,
        });
    }

    Ok(result)
}

fn remove_worktree(wt: &Worktree) -> Result<()> {
    let status = Command::new("git")
        .args(["worktree", "remove", "--force"])
        .arg(&wt.path)
        .status()?;

    if !status.success() {
        println!(
            "{} {}",
            "Failed:".red(),
            wt.path.display()
        );
    } else {
        println!(
            "{} {}",
            "Deleted:".green(),
            wt.path.display()
        );
    }

    Ok(())
}
