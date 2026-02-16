use git2::{Diff, DiffFormat, DiffOptions, Repository, StatusOptions};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatusKind {
    Added,
    Modified,
    Deleted,
    Renamed,
    Untracked,
}

#[derive(Debug, Clone)]
pub struct GitFileEntry {
    pub path: String,
    pub status: FileStatusKind,
    pub staged: bool,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Context,
    Addition,
    Deletion,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: String,
    pub old_path: Option<String>,
    pub hunks: Vec<DiffHunk>,
    pub is_binary: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GitSummary {
    pub additions: usize,
    pub deletions: usize,
    pub changed_files: usize,
    pub branch: String,
}

pub struct GitService;

impl GitService {
    pub fn open(path: &Path) -> Result<Repository, git2::Error> {
        Repository::discover(path)
    }

    pub fn current_branch(repo: &Repository) -> String {
        repo.head()
            .ok()
            .and_then(|head| head.shorthand().map(String::from))
            .unwrap_or_else(|| "HEAD".to_string())
    }

    pub fn status_entries(repo: &Repository) -> Result<Vec<GitFileEntry>, git2::Error> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_unmodified(false);

        let statuses = repo.statuses(Some(&mut opts))?;
        let mut entries = Vec::new();

        for entry in statuses.iter() {
            let path = entry.path().unwrap_or("").to_string();
            let s = entry.status();

            if s.is_index_new() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Added,
                    staged: true,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_index_modified() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Modified,
                    staged: true,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_index_deleted() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Deleted,
                    staged: true,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_index_renamed() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Renamed,
                    staged: true,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_wt_new() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Untracked,
                    staged: false,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_wt_modified() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Modified,
                    staged: false,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_wt_deleted() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Deleted,
                    staged: false,
                    additions: 0,
                    deletions: 0,
                });
            }
            if s.is_wt_renamed() {
                entries.push(GitFileEntry {
                    path: path.clone(),
                    status: FileStatusKind::Renamed,
                    staged: false,
                    additions: 0,
                    deletions: 0,
                });
            }
        }

        Self::fill_line_counts(repo, &mut entries);
        Self::dedup_entries(&mut entries);
        entries.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(entries)
    }

    fn fill_line_counts(repo: &Repository, entries: &mut [GitFileEntry]) {
        for entry in entries.iter_mut() {
            let diff_result = if entry.staged {
                Self::diff_staged_for_path(repo, &entry.path)
            } else {
                Self::diff_workdir_for_path(repo, &entry.path)
            };
            if let Ok(diff) = diff_result {
                if let Ok(stats) = diff.stats() {
                    entry.additions = stats.insertions();
                    entry.deletions = stats.deletions();
                }
            }
        }
    }

    fn dedup_entries(entries: &mut Vec<GitFileEntry>) {
        let mut seen = std::collections::HashMap::new();
        let mut result = Vec::new();
        for entry in entries.drain(..) {
            let key = (entry.path.clone(), entry.staged);
            if !seen.contains_key(&key) {
                seen.insert(key, true);
                result.push(entry);
            }
        }
        *entries = result;
    }

    pub fn summary(repo: &Repository) -> GitSummary {
        let branch = Self::current_branch(repo);
        let entries = Self::status_entries(repo).unwrap_or_default();
        let mut additions = 0;
        let mut deletions = 0;
        for entry in &entries {
            additions += entry.additions;
            deletions += entry.deletions;
        }
        GitSummary {
            additions,
            deletions,
            changed_files: entries.len(),
            branch,
        }
    }

    pub fn file_diff_workdir(repo: &Repository, path: &str) -> Result<FileDiff, git2::Error> {
        let diff = Self::diff_workdir_for_path(repo, path)?;
        Self::parse_diff(&diff, path)
    }

    pub fn file_diff_staged(repo: &Repository, path: &str) -> Result<FileDiff, git2::Error> {
        let diff = Self::diff_staged_for_path(repo, path)?;
        Self::parse_diff(&diff, path)
    }

    fn diff_workdir_for_path<'a>(
        repo: &'a Repository,
        path: &str,
    ) -> Result<Diff<'a>, git2::Error> {
        let mut opts = DiffOptions::new();
        opts.pathspec(path)
            .include_untracked(true)
            .show_untracked_content(true);
        let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());
        repo.diff_tree_to_workdir_with_index(head_tree.as_ref(), Some(&mut opts))
    }

    fn diff_staged_for_path<'a>(repo: &'a Repository, path: &str) -> Result<Diff<'a>, git2::Error> {
        let mut opts = DiffOptions::new();
        opts.pathspec(path);
        let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());
        let index = repo.index()?;
        repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), Some(&mut opts))
    }

    fn parse_diff(diff: &Diff<'_>, path: &str) -> Result<FileDiff, git2::Error> {
        let mut file_diff = FileDiff {
            path: path.to_string(),
            old_path: None,
            hunks: Vec::new(),
            is_binary: false,
        };

        let num_deltas = diff.deltas().len();
        for delta_idx in 0..num_deltas {
            let delta = diff.deltas().nth(delta_idx);
            if let Some(delta) = delta {
                if delta.flags().is_binary() {
                    file_diff.is_binary = true;
                    return Ok(file_diff);
                }
                if let Some(old) = delta.old_file().path() {
                    let old_str = old.to_string_lossy().to_string();
                    if old_str != path {
                        file_diff.old_path = Some(old_str);
                    }
                }
            }
        }

        let mut current_hunk_lines: Vec<DiffLine> = Vec::new();
        let mut in_hunk = false;

        diff.print(DiffFormat::Patch, |_delta, hunk, line| {
            if hunk.is_some() {
                if in_hunk {
                    file_diff.hunks.push(DiffHunk {
                        lines: std::mem::take(&mut current_hunk_lines),
                    });
                }
                in_hunk = true;
            }

            let content = String::from_utf8_lossy(line.content())
                .trim_end_matches('\n')
                .trim_end_matches('\r')
                .to_string();

            match line.origin() {
                '+' => current_hunk_lines.push(DiffLine {
                    kind: DiffLineKind::Addition,
                    old_lineno: None,
                    new_lineno: line.new_lineno(),
                    content,
                }),
                '-' => current_hunk_lines.push(DiffLine {
                    kind: DiffLineKind::Deletion,
                    old_lineno: line.old_lineno(),
                    new_lineno: None,
                    content,
                }),
                ' ' => current_hunk_lines.push(DiffLine {
                    kind: DiffLineKind::Context,
                    old_lineno: line.old_lineno(),
                    new_lineno: line.new_lineno(),
                    content,
                }),
                _ => {}
            }
            true
        })?;

        if in_hunk {
            file_diff.hunks.push(DiffHunk {
                lines: current_hunk_lines,
            });
        }

        Ok(file_diff)
    }

    pub fn stage_file(repo: &Repository, path: &str) -> Result<(), git2::Error> {
        let mut index = repo.index()?;
        let abs_path = repo.workdir().unwrap_or(Path::new(".")).join(path);
        if abs_path.exists() {
            index.add_path(Path::new(path))?;
        } else {
            index.remove_path(Path::new(path))?;
        }
        index.write()?;
        Ok(())
    }

    pub fn unstage_file(repo: &Repository, path: &str) -> Result<(), git2::Error> {
        let commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());

        match commit {
            Some(target) => {
                repo.reset_default(Some(&target.into_object()), [path])?;
            }
            None => {
                let mut index = repo.index()?;
                index.remove_path(Path::new(path))?;
                index.write()?;
            }
        }
        Ok(())
    }

    pub fn commit(repo: &Repository, message: &str) -> Result<git2::Oid, git2::Error> {
        let mut index = repo.index()?;
        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid)?;
        let sig = repo.signature()?;
        let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
        let parents: Vec<&git2::Commit<'_>> = parent.as_ref().map(|c| vec![c]).unwrap_or_default();
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
    }

    pub fn read_head_content(repo: &Repository, path: &str) -> Option<String> {
        let tree = repo.head().ok()?.peel_to_tree().ok()?;
        let entry = tree.get_path(Path::new(path)).ok()?;
        let blob = entry.to_object(repo).ok()?.peel_to_blob().ok()?;
        std::str::from_utf8(blob.content()).ok().map(String::from)
    }

    pub fn read_workdir_content(repo: &Repository, path: &str) -> Option<String> {
        let workdir = repo.workdir()?;
        std::fs::read_to_string(workdir.join(path)).ok()
    }

    pub fn file_diff_untracked(repo: &Repository, path: &str) -> Result<FileDiff, git2::Error> {
        let content = Self::read_workdir_content(repo, path).unwrap_or_default();
        let mut hunks = Vec::new();
        let lines: Vec<DiffLine> = content
            .lines()
            .enumerate()
            .map(|(i, line)| DiffLine {
                kind: DiffLineKind::Addition,
                old_lineno: None,
                new_lineno: Some(i as u32 + 1),
                content: line.to_string(),
            })
            .collect();

        if !lines.is_empty() {
            hunks.push(DiffHunk { lines });
        }

        Ok(FileDiff {
            path: path.to_string(),
            old_path: None,
            hunks,
            is_binary: false,
        })
    }
}
