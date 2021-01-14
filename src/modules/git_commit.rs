use super::{Context, Module, RootModuleConfig};

use crate::configs::git_commit::GitCommitConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git commit in the current directory
///
/// Will display the commit hash if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_commit");
    let config: GitCommitConfig = GitCommitConfig::try_load(module.config);

    let repo = context.repo().as_ref()?;

    let mut parsed;

    parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hash" => repo
                    .commit_hash().as_ref()
                    .map(|h| {
                        h.chars()
                            .take(config.commit_hash_length)
                            .collect::<String>()
                    })
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    if !config.tag_disabled {
        let tag_name = repo.commit_tag();

        if let Some(tag_name) = tag_name {
            parsed = StringFormatter::new(config.format).and_then(|formatter| {
                formatter
                    .map_style(|variable| match variable {
                        "style" => Some(Ok(config.style)),
                        _ => None,
                    })
                    .map(|variable| match variable {
                        "hash" => repo.commit_hash().as_ref().map(|hash| {
                            Ok(hash
                                .chars()
                                .take(config.commit_hash_length)
                                .collect::<String>())
                        }),
                        _ => None,
                    })
                    .map(|variable| match variable {
                        "tag" => Some(Ok(format!(" {}{}", &config.tag_symbol, &tag_name))),
                        _ => None,
                    })
                    .parse(None)
            });
        }
    };

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_commit`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;
    use std::process::Command;
    use std::{io, str};

    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_len_override() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(14);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    commit_hash_length = 14
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        Command::new("git")
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_output.truncate(7);
        let expected_hash = str::from_utf8(&git_output).unwrap();

        let actual = ModuleRenderer::new("git_commit")
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_hash))
                .to_string()
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_commit_hash_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_commit = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = Command::new("git")
            .args(&["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{} {}", commit_output, tag_output);

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = ""
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
                .to_string()
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_render_commit_hash_only_detached_on_detached_with_tag_enabled() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        Command::new("git")
            .args(&["checkout", "@~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        Command::new("git")
            .args(&["tag", "tagOnDetached", "-m", "Testing tags on detached"])
            .current_dir(&repo_dir.path())
            .output()?;

        let mut git_commit = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        let git_tag = Command::new("git")
            .args(&["describe", "--tags", "--exact-match", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{} {}", commit_output, tag_output);

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    tag_disabled = false
                    tag_symbol = ""
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
                .to_string()
        ));

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_latest_tag_shown_with_tag_enabled() -> io::Result<()> {
        use std::{thread, time};

        let repo_dir = fixture_repo(FixtureProvider::GIT)?;

        let mut git_commit = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        git_commit.truncate(7);
        let commit_output = str::from_utf8(&git_commit).unwrap().trim();

        Command::new("git")
            .args(&["tag", "v2", "-m", "Testing tags v2"])
            .current_dir(&repo_dir.path())
            .output()?;

        // Wait one second between tags
        thread::sleep(time::Duration::from_millis(1000));

        Command::new("git")
            .args(&["tag", "v0", "-m", "Testing tags v0", "HEAD~1"])
            .current_dir(&repo_dir.path())
            .output()?;

        Command::new("git")
            .args(&["tag", "v1", "-m", "Testing tags v1"])
            .current_dir(&repo_dir.path())
            .output()?;

        let git_tag = Command::new("git")
            .args(&[
                "for-each-ref",
                "--contains",
                "HEAD",
                "--sort=-taggerdate",
                "--count=1",
                "--format",
                "%(refname:short)",
                "refs/tags",
            ])
            .current_dir(&repo_dir.path())
            .output()?
            .stdout;
        let tag_output = str::from_utf8(&git_tag).unwrap().trim();

        let expected_output = format!("{} {}", commit_output, tag_output);

        let actual = ModuleRenderer::new("git_commit")
            .config(toml::toml! {
                [git_commit]
                    only_detached = false
                    tag_disabled = false
                    tag_symbol = ""
            })
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green
                .bold()
                .paint(format!("({})", expected_output.trim()))
                .to_string()
        ));

        assert_eq!(expected, actual);
        Ok(())
    }
}
