use once_cell::sync::OnceCell;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils;

#[derive(Default, Debug, PartialEq)]
pub struct GitStatus {
    pub untracked: usize,
    pub added: usize,
    pub modified: usize,
    pub renamed: usize,
    pub deleted: usize,
    pub stashed: usize,
    pub unmerged: usize,
    pub ahead: usize,
    pub behind: usize,
    pub diverged: usize,
    pub conflicted: usize,
    pub staged: usize,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GitState {
    Clean,
    Merge,
    Revert,
    CherryPick,
    Bisect,
    ApplyMailbox,
    ApplyMailboxOrRebase,
    Rebase(RebaseProgress),
}

#[derive(PartialEq, Eq, Debug)]
pub struct RebaseProgress {
    pub current: usize,
    pub end: usize,
}

#[derive(Debug, Default)]
pub struct Repository {
    pub git_dir: PathBuf,
    pub root_dir: PathBuf,
    branch: OnceCell<String>,
    status: OnceCell<GitStatus>,
    state: OnceCell<GitState>,
    hash: OnceCell<Option<String>>,
    remote: OnceCell<Option<String>>,
    tag: OnceCell<Option<String>>,
}

impl Repository {
    /// Search up the directory tree for ".git" directories to identify git root
    pub fn discover(path: &Path) -> Option<Self> {
        log::trace!("Checking for Git instance: {:?}", path);
        if let Some(repository) = Repository::scan(path) {
            return Some(repository);
        }

        match path.parent() {
            Some(parent) => Repository::discover(parent),
            None => None,
        }
    }

    /// Check whether a given path is a git directory
    fn scan(path: &Path) -> Option<Self> {
        let git_dir = path.join(".git");
        if !git_dir.exists() {
            return None;
        }

        log::trace!("Git repository found");
        Some(Repository {
            git_dir,
            root_dir: path.into(),
            ..Default::default()
        })
    }

    /// Get the status of the current git repo
    pub fn status(&self) -> &GitStatus {
        self.status.get_or_init(|| self.get_status())
    }

    fn get_status(&self) -> GitStatus {
        let output = match utils::exec_cmd(
            "git",
            &[
                "--git-dir",
                self.git_dir.to_str().unwrap(),
                "status",
                "--porcelain",
            ],
        ) {
            Some(output) => output.stdout,
            None => return Default::default(),
        };
        parse_porcelain_output(output)
    }

    /// Get the branch name of the current git repo
    pub fn branch(&self) -> &String {
        self.branch.get_or_init(|| match self.get_branch() {
            Some(branch) => branch,
            None => String::from("HEAD"),
        })
    }

    fn get_branch(&self) -> Option<String> {
        let head_file = self.git_dir.join("HEAD");
        let head_contents = fs::read_to_string(head_file).ok()?;
        let branch_start = head_contents.rfind('/')?;
        let branch_name = &head_contents[branch_start + 1..];
        let trimmed_branch_name = branch_name.trim_end();
        Some(trimmed_branch_name.into())
    }

    /// Get the remote name of the current git repo
    pub fn remote(&self) -> &Option<String> {
        self.remote.get_or_init(|| self.get_remote())
    }

    fn get_remote(&self) -> Option<String> {
        let stdout = utils::exec_cmd(
            "git",
            &["--git-dir", self.git_dir.to_str().unwrap(), "remote"],
        )?
        .stdout;

        Some(stdout).filter(|s| !s.is_empty())
    }

    /// Get the state of the current git repo
    pub fn state(&self) -> &GitState {
        self.state.get_or_init(|| self.get_state())
    }

    // Loosely ported from git.git
    // https://github.com/git/git/blob/master/contrib/completion/git-prompt.sh#L446 
    fn get_state(&self) -> GitState {
        let merge_file = self.git_dir.join("MERGE_HEAD");
        if merge_file.exists() {
            return GitState::Merge;
        }

        let bisect_file = self.git_dir.join("BISECT_LOG");
        if bisect_file.exists() {
            return GitState::Bisect;
        }
        
        let rebase_merge_dir = self.git_dir.join("rebase-merge");
        if rebase_merge_dir.exists() {
            let rebase_progress = self
                .get_rebase_progress()
                .unwrap_or_else(|| RebaseProgress { current: 1, end: 1 });
            return GitState::Rebase(rebase_progress);
        }

        GitState::Clean
    }

    fn get_rebase_progress(&self) -> Option<RebaseProgress> {
        let has_path = |relative_path: &str| {
            let path = self.git_dir.join(PathBuf::from(relative_path));
            path.exists()
        };

        let file_to_usize = |relative_path: &str| {
            let path = self.git_dir.join(PathBuf::from(relative_path));
            let contents = crate::utils::read_file(path).ok()?;
            let quantity = contents.trim().parse::<usize>().ok()?;
            Some(quantity)
        };

        let paths_to_progress = |current_path: &str, total_path: &str| {
            let current = file_to_usize(current_path)?;
            let end = file_to_usize(total_path)?;
            Some(RebaseProgress { current, end })
        };

        if has_path("rebase-merge/msgnum") {
            paths_to_progress("rebase-merge/msgnum", "rebase-merge/end")
        } else if has_path("rebase-merge/onto") {
            Some(RebaseProgress { current: 1, end: 1 })
        } else if has_path("rebase-apply") {
            paths_to_progress("rebase-apply/next", "rebase-apply/last")
        } else {
            None
        }
    }

    /// Get the hash of the active commit on the current git repo
    pub fn commit_hash(&self) -> &Option<String> {
        self.hash.get_or_init(|| self.get_commit_hash())
    }

    fn get_commit_hash(&self) -> Option<String> {
        let output = utils::exec_cmd(
            "git",
            &[
                "--git-dir",
                self.git_dir.to_str().unwrap(),
                "rev-parse",
                "HEAD",
            ],
        )?;
        Some(output.stdout)
    }

    /// Get the tag of the active commit on the current git repo
    pub fn commit_tag(&self) -> &Option<String> {
        self.tag.get_or_init(|| self.get_commit_tag())
    }

    fn get_commit_tag(&self) -> Option<String> {
        // TODO: Actually get the tag
        None
    }
}

/// Parse git status values from `git status --porcelain`
///
/// Example porcelain output:
/// ```code
///  M src/prompt.rs
///  M src/main.rs
/// ```
fn parse_porcelain_output<S: Into<String>>(porcelain: S) -> GitStatus {
    let porcelain_str = porcelain.into();
    let porcelain_lines = porcelain_str.lines();
    let mut vcs_status: GitStatus = Default::default();

    porcelain_lines.for_each(|line| {
        let mut characters = line.chars();

        // Extract the first two letter of each line
        let letter_codes = (
            characters.next().unwrap_or(' '),
            characters.next().unwrap_or(' '),
        );

        // TODO: Revisit conflict and staged logic
        if letter_codes.0 == letter_codes.1 {
            vcs_status.conflicted += 1
        } else {
            increment_git_status(&mut vcs_status, 'S');
            increment_git_status(&mut vcs_status, letter_codes.1);
        }
    });

    vcs_status
}

/// Update the cumulative git status, given the "short format" letter of a file's status
/// https://git-scm.com/docs/git-status#_short_format
fn increment_git_status(vcs_status: &mut GitStatus, letter: char) {
    match letter {
        'A' => vcs_status.added += 1,
        'M' => vcs_status.modified += 1,
        'D' => vcs_status.deleted += 1,
        'R' => vcs_status.renamed += 1,
        'C' => vcs_status.added += 1,
        'U' => vcs_status.modified += 1,
        '?' => vcs_status.untracked += 1,
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_parse_empty_porcelain_output() -> io::Result<()> {
        let output = parse_porcelain_output("");

        let expected: GitStatus = Default::default();
        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_parse_porcelain_output() -> io::Result<()> {
        let output = parse_porcelain_output(
            "M src/prompt.rs
MM src/main.rs
A src/formatter.rs
? README.md",
        );

        let expected = GitStatus {
            modified: 2,
            added: 1,
            untracked: 1,
            ..Default::default()
        };
        assert_eq!(output, expected);
        Ok(())
    }
}
