use git2::{ApplyLocation, Diff, DiffFormat, DiffOptions, Repository, StatusOptions};
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
#[allow(dead_code)]
pub struct DiffHunk {
    pub header: String,
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
    pub hunk_index: usize,
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
        let mut current_hunk_header = String::new();
        let mut current_hunk_old_start = 0u32;
        let mut current_hunk_old_lines = 0u32;
        let mut current_hunk_new_start = 0u32;
        let mut current_hunk_new_lines = 0u32;
        let mut hunk_count = 0usize;
        let mut in_hunk = false;

        diff.print(DiffFormat::Patch, |_delta, hunk, line| {
            if let Some(hunk) = hunk {
                if in_hunk {
                    file_diff.hunks.push(DiffHunk {
                        header: current_hunk_header.clone(),
                        old_start: current_hunk_old_start,
                        old_lines: current_hunk_old_lines,
                        new_start: current_hunk_new_start,
                        new_lines: current_hunk_new_lines,
                        lines: std::mem::take(&mut current_hunk_lines),
                        hunk_index: hunk_count,
                    });
                    hunk_count += 1;
                }
                current_hunk_header = String::from_utf8_lossy(hunk.header()).trim().to_string();
                current_hunk_old_start = hunk.old_start();
                current_hunk_old_lines = hunk.old_lines();
                current_hunk_new_start = hunk.new_start();
                current_hunk_new_lines = hunk.new_lines();
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
                header: current_hunk_header,
                old_start: current_hunk_old_start,
                old_lines: current_hunk_old_lines,
                new_start: current_hunk_new_start,
                new_lines: current_hunk_new_lines,
                lines: current_hunk_lines,
                hunk_index: hunk_count,
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

    pub fn stage_hunk(repo: &Repository, path: &str, hunk_idx: usize) -> Result<(), git2::Error> {
        let diff = Self::diff_workdir_for_path(repo, path)?;
        let patch = git2::Patch::from_diff(&diff, 0)?;
        if let Some(patch) = patch {
            let hunk_diff = Self::build_single_hunk_patch(&patch, hunk_idx)?;
            let diff = Diff::from_buffer(hunk_diff.as_bytes())?;
            repo.apply(&diff, ApplyLocation::Index, None)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn discard_hunk(repo: &Repository, path: &str, hunk_idx: usize) -> Result<(), git2::Error> {
        let diff = Self::diff_workdir_for_path(repo, path)?;
        let patch = git2::Patch::from_diff(&diff, 0)?;
        if let Some(patch) = patch {
            let hunk_diff = Self::build_single_hunk_patch(&patch, hunk_idx)?;
            let mut reversed = String::new();
            for line in hunk_diff.lines() {
                if line.starts_with("--- ") {
                    reversed.push_str(line);
                    reversed.push('\n');
                } else if line.starts_with("+++ ") {
                    reversed.push_str(line);
                    reversed.push('\n');
                } else if line.starts_with('+') {
                    reversed.push('-');
                    reversed.push_str(&line[1..]);
                    reversed.push('\n');
                } else if line.starts_with('-') {
                    reversed.push('+');
                    reversed.push_str(&line[1..]);
                    reversed.push('\n');
                } else if line.starts_with("@@") {
                    let reversed_header = Self::reverse_hunk_header(line);
                    reversed.push_str(&reversed_header);
                    reversed.push('\n');
                } else {
                    reversed.push_str(line);
                    reversed.push('\n');
                }
            }
            let diff = Diff::from_buffer(reversed.as_bytes())?;
            repo.apply(&diff, ApplyLocation::WorkDir, None)?;
        }
        Ok(())
    }

    fn build_single_hunk_patch(
        patch: &git2::Patch<'_>,
        hunk_idx: usize,
    ) -> Result<String, git2::Error> {
        let delta = patch.delta();
        let old_path = delta
            .old_file()
            .path()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let new_path = delta
            .new_file()
            .path()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let (hunk, _num_lines) = patch.hunk(hunk_idx)?;
        let num_lines = patch.num_lines_in_hunk(hunk_idx)?;

        let mut out = String::new();
        out.push_str(&format!("--- a/{}\n", old_path));
        out.push_str(&format!("+++ b/{}\n", new_path));
        out.push_str(&String::from_utf8_lossy(hunk.header()));

        for line_idx in 0..num_lines {
            let line = patch.line_in_hunk(hunk_idx, line_idx)?;
            match line.origin() {
                '+' | '-' | ' ' => {
                    out.push(line.origin());
                    out.push_str(&String::from_utf8_lossy(line.content()));
                }
                _ => {
                    out.push_str(&String::from_utf8_lossy(line.content()));
                }
            }
        }

        Ok(out)
    }

    #[allow(dead_code)]
    fn reverse_hunk_header(header: &str) -> String {
        let trimmed = header.trim();
        if !trimmed.starts_with("@@") {
            return header.to_string();
        }

        let inner = trimmed
            .trim_start_matches("@@")
            .trim_end_matches("@@")
            .trim();
        let parts: Vec<&str> = inner.split_whitespace().collect();
        if parts.len() >= 2 {
            format!("@@ {} {} @@", parts[1], parts[0])
        } else {
            header.to_string()
        }
    }

    #[allow(dead_code)]
    pub fn discard_file(repo: &Repository, path: &str) -> Result<(), git2::Error> {
        let obj = repo.head()?.peel_to_tree()?.into_object();
        repo.checkout_tree(
            &obj,
            Some(git2::build::CheckoutBuilder::new().path(path).force()),
        )?;
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
            hunks.push(DiffHunk {
                header: String::new(),
                old_start: 0,
                old_lines: 0,
                new_start: 1,
                new_lines: lines.len() as u32,
                lines,
                hunk_index: 0,
            });
        }

        Ok(FileDiff {
            path: path.to_string(),
            old_path: None,
            hunks,
            is_binary: false,
        })
    }
}
