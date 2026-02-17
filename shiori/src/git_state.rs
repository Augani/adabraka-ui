use crate::diff_highlighter::{compute_line_highlights, HighlightRun};
use crate::git_service::{
    DiffLine, DiffLineKind, FileDiff, FileStatusKind, GitFileEntry, GitService, GitSummary,
};
use adabraka_ui::components::editor::{EditorState, Language};
use gpui::*;
use gpui::UniformListScrollHandle;
use smol::Timer;
use std::path::PathBuf;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffViewMode {
    Split,
    Unified,
}

#[derive(Debug, Clone)]
pub struct DiffRow {
    pub left: Option<DiffLine>,
    pub right: Option<DiffLine>,
    pub left_highlights: Vec<HighlightRun>,
    pub right_highlights: Vec<HighlightRun>,
}

pub struct GitState {
    pub repo_path: Option<PathBuf>,
    pub file_entries: Vec<GitFileEntry>,
    pub selected_file_index: usize,
    pub active_diff: Option<FileDiff>,
    pub aligned_rows: Vec<DiffRow>,
    pub commit_editor: Entity<EditorState>,
    pub diff_split_pct: f32,
    pub summary: GitSummary,
    polling_task: Option<Task<()>>,
    pub loading: bool,
    pub error_message: Option<String>,
    pub diff_view_mode: DiffViewMode,
    old_line_highlights: Vec<Vec<HighlightRun>>,
    new_line_highlights: Vec<Vec<HighlightRun>>,
    pub diff_scroll_handle: UniformListScrollHandle,
    pub file_list_scroll_handle: ScrollHandle,
}

impl GitState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let commit_editor = cx.new(|cx| EditorState::new(cx));
        Self {
            repo_path: None,
            file_entries: Vec::new(),
            selected_file_index: 0,
            active_diff: None,
            aligned_rows: Vec::new(),
            commit_editor,
            diff_split_pct: 0.5,
            summary: GitSummary::default(),
            polling_task: None,
            loading: false,
            error_message: None,
            diff_view_mode: DiffViewMode::Split,
            old_line_highlights: Vec::new(),
            new_line_highlights: Vec::new(),
            diff_scroll_handle: UniformListScrollHandle::new(),
            file_list_scroll_handle: ScrollHandle::new(),
        }
    }

    pub fn set_diff_view_mode(&mut self, mode: DiffViewMode, cx: &mut Context<Self>) {
        if self.diff_view_mode == mode {
            return;
        }
        self.diff_view_mode = mode;
        if let Some(diff) = &self.active_diff {
            self.aligned_rows = Self::build_aligned_rows(
                diff,
                self.diff_view_mode,
                &self.old_line_highlights,
                &self.new_line_highlights,
            );
        }
        cx.notify();
    }

    pub fn set_diff_split_pct(&mut self, pct: f32, cx: &mut Context<Self>) {
        self.diff_split_pct = pct.clamp(0.2, 0.8);
        cx.notify();
    }

    pub fn stage_all(&mut self, cx: &mut Context<Self>) {
        self.stage_filtered(|e| !e.staged, "Stage all failed", cx);
    }

    fn stage_filtered(
        &mut self,
        filter: impl Fn(&GitFileEntry) -> bool,
        error_prefix: &str,
        cx: &mut Context<Self>,
    ) {
        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        let paths: Vec<String> = self
            .file_entries
            .iter()
            .filter(|e| filter(e))
            .map(|e| e.path.clone())
            .collect();

        if paths.is_empty() {
            return;
        }

        let prefix = error_prefix.to_string();
        cx.spawn(async move |this, cx| {
            let p = repo_path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&p)?;
                for path in &paths {
                    GitService::stage_file(&repo, path)?;
                }
                Ok::<(), git2::Error>(())
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    if let Err(e) = result {
                        state.error_message = Some(format!("{}: {}", prefix, e));
                    }
                    state.refresh(cx);
                });
            });
        })
        .detach();
    }

    pub fn unstage_all(&mut self, cx: &mut Context<Self>) {
        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        let staged: Vec<String> = self
            .file_entries
            .iter()
            .filter(|e| e.staged)
            .map(|e| e.path.clone())
            .collect();

        if staged.is_empty() {
            return;
        }

        cx.spawn(async move |this, cx| {
            let p = repo_path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&p)?;
                for path in &staged {
                    GitService::unstage_file(&repo, path)?;
                }
                Ok::<(), git2::Error>(())
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    if let Err(e) = result {
                        state.error_message = Some(format!("Unstage all failed: {}", e));
                    }
                    state.refresh(cx);
                });
            });
        })
        .detach();
    }

    pub fn set_workspace(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.repo_path = Some(path);
        self.refresh(cx);
        self.start_polling(cx);
    }

    pub fn start_polling(&mut self, cx: &mut Context<Self>) {
        if self.polling_task.is_some() {
            return;
        }
        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        let task = cx.spawn(async move |this, cx| loop {
            Timer::after(POLL_INTERVAL).await;
            let path = repo_path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&path).ok()?;
                let entries = GitService::status_entries(&repo).ok()?;
                let summary = GitService::summary(&repo);
                Some((entries, summary))
            })
            .await;

            let should_continue = cx
                .update(|cx| {
                    let _ = this.update(cx, |state, cx| {
                        if let Some((entries, summary)) = result {
                            let changed = state.file_entries.len() != entries.len()
                                || state.summary.changed_files != summary.changed_files
                                || state.summary.additions != summary.additions
                                || state.summary.deletions != summary.deletions;
                            state.file_entries = entries;
                            state.summary = summary;
                            if changed {
                                if state.selected_file_index >= state.file_entries.len() {
                                    state.selected_file_index =
                                        state.file_entries.len().saturating_sub(1);
                                }
                                state.load_selected_diff(cx);
                            }
                            cx.notify();
                        }
                    });
                })
                .is_ok();
            if !should_continue {
                break;
            }
        });

        self.polling_task = Some(task);
    }

    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        self.loading = true;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let path = repo_path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&path).ok()?;
                let entries = GitService::status_entries(&repo).ok()?;
                let summary = GitService::summary(&repo);
                Some((entries, summary))
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    state.loading = false;
                    if let Some((entries, summary)) = result {
                        state.file_entries = entries;
                        state.summary = summary;
                        if state.selected_file_index >= state.file_entries.len() {
                            state.selected_file_index = state.file_entries.len().saturating_sub(1);
                        }
                        state.load_selected_diff(cx);
                    }
                    cx.notify();
                });
            });
        })
        .detach();
    }

    pub fn select_file(&mut self, idx: usize, cx: &mut Context<Self>) {
        if idx >= self.file_entries.len() {
            return;
        }
        self.selected_file_index = idx;
        self.load_selected_diff(cx);
        cx.notify();
    }

    pub fn select_next_file(&mut self, cx: &mut Context<Self>) {
        if self.file_entries.is_empty() {
            return;
        }
        let next = (self.selected_file_index + 1) % self.file_entries.len();
        self.select_file(next, cx);
    }

    pub fn select_prev_file(&mut self, cx: &mut Context<Self>) {
        if self.file_entries.is_empty() {
            return;
        }
        let prev = if self.selected_file_index == 0 {
            self.file_entries.len() - 1
        } else {
            self.selected_file_index - 1
        };
        self.select_file(prev, cx);
    }

    fn load_selected_diff(&mut self, cx: &mut Context<Self>) {
        let entry = match self.file_entries.get(self.selected_file_index) {
            Some(e) => e.clone(),
            None => {
                self.active_diff = None;
                self.aligned_rows.clear();
                self.old_line_highlights.clear();
                self.new_line_highlights.clear();
                return;
            }
        };

        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        let path = entry.path.clone();
        let staged = entry.staged;
        let is_untracked = entry.status == FileStatusKind::Untracked;

        cx.spawn(async move |this, cx| {
            let p = repo_path.clone();
            let file_path = path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&p).ok()?;
                let diff = if is_untracked {
                    GitService::file_diff_untracked(&repo, &file_path).ok()
                } else if staged {
                    GitService::file_diff_staged(&repo, &file_path).ok()
                } else {
                    GitService::file_diff_workdir(&repo, &file_path).ok()
                };

                let old_content = if is_untracked {
                    None
                } else {
                    GitService::read_head_content(&repo, &file_path)
                };
                let new_content = GitService::read_workdir_content(&repo, &file_path);

                // Compute syntax highlighting in background thread (not on UI thread)
                let (old_highlights, new_highlights) = if let Some(ref diff) = diff {
                    let lang = Language::from_path(&std::path::Path::new(&diff.path));
                    let old_hl = old_content
                        .as_deref()
                        .map(|c| compute_line_highlights(c, lang))
                        .unwrap_or_default();
                    let new_hl = new_content
                        .as_deref()
                        .map(|c| compute_line_highlights(c, lang))
                        .unwrap_or_default();
                    (old_hl, new_hl)
                } else {
                    (Vec::new(), Vec::new())
                };

                Some((diff, old_highlights, new_highlights))
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    if let Some((diff_opt, old_highlights, new_highlights)) = result {
                        if let Some(diff) = diff_opt {
                            state.old_line_highlights = old_highlights;
                            state.new_line_highlights = new_highlights;
                            state.aligned_rows = Self::build_aligned_rows(
                                &diff,
                                state.diff_view_mode,
                                &state.old_line_highlights,
                                &state.new_line_highlights,
                            );
                            state.active_diff = Some(diff);
                        } else {
                            state.active_diff = None;
                            state.aligned_rows.clear();
                            state.old_line_highlights.clear();
                            state.new_line_highlights.clear();
                        }
                    }
                    cx.notify();
                });
            });
        })
        .detach();
    }

    fn build_aligned_rows(
        diff: &FileDiff,
        mode: DiffViewMode,
        old_highlights: &[Vec<HighlightRun>],
        new_highlights: &[Vec<HighlightRun>],
    ) -> Vec<DiffRow> {
        match mode {
            DiffViewMode::Split => Self::build_split_rows(diff, old_highlights, new_highlights),
            DiffViewMode::Unified => Self::build_unified_rows(diff, old_highlights, new_highlights),
        }
    }

    fn get_highlights(
        line: &DiffLine,
        old_highlights: &[Vec<HighlightRun>],
        new_highlights: &[Vec<HighlightRun>],
    ) -> Vec<HighlightRun> {
        match line.kind {
            DiffLineKind::Deletion | DiffLineKind::Context => {
                if let Some(lineno) = line.old_lineno {
                    let idx = lineno as usize - 1;
                    old_highlights.get(idx).cloned().unwrap_or_default()
                } else {
                    Vec::new()
                }
            }
            DiffLineKind::Addition => {
                if let Some(lineno) = line.new_lineno {
                    let idx = lineno as usize - 1;
                    new_highlights.get(idx).cloned().unwrap_or_default()
                } else {
                    Vec::new()
                }
            }
        }
    }

    fn build_split_rows(
        diff: &FileDiff,
        old_highlights: &[Vec<HighlightRun>],
        new_highlights: &[Vec<HighlightRun>],
    ) -> Vec<DiffRow> {
        let mut rows = Vec::new();

        for hunk in &diff.hunks {
            let mut del_buf: Vec<(DiffLine, Vec<HighlightRun>)> = Vec::new();
            let mut add_buf: Vec<(DiffLine, Vec<HighlightRun>)> = Vec::new();

            for line in &hunk.lines {
                match line.kind {
                    DiffLineKind::Context => {
                        Self::flush_split(&mut del_buf, &mut add_buf, &mut rows);
                        let hl = Self::get_highlights(line, old_highlights, new_highlights);
                        rows.push(DiffRow {
                            left: Some(line.clone()),
                            right: Some(line.clone()),
                            left_highlights: hl.clone(),
                            right_highlights: hl,
                        });
                    }
                    DiffLineKind::Deletion => {
                        let hl = Self::get_highlights(line, old_highlights, new_highlights);
                        del_buf.push((line.clone(), hl));
                    }
                    DiffLineKind::Addition => {
                        let hl = Self::get_highlights(line, old_highlights, new_highlights);
                        add_buf.push((line.clone(), hl));
                    }
                }
            }
            Self::flush_split(&mut del_buf, &mut add_buf, &mut rows);
        }

        rows
    }

    fn flush_split(
        del_buf: &mut Vec<(DiffLine, Vec<HighlightRun>)>,
        add_buf: &mut Vec<(DiffLine, Vec<HighlightRun>)>,
        rows: &mut Vec<DiffRow>,
    ) {
        let max_len = del_buf.len().max(add_buf.len());
        for i in 0..max_len {
            let (left, left_hl) = del_buf
                .get(i)
                .map(|(l, h)| (Some(l.clone()), h.clone()))
                .unwrap_or((None, Vec::new()));
            let (right, right_hl) = add_buf
                .get(i)
                .map(|(l, h)| (Some(l.clone()), h.clone()))
                .unwrap_or((None, Vec::new()));
            rows.push(DiffRow {
                left,
                right,
                left_highlights: left_hl,
                right_highlights: right_hl,
            });
        }
        del_buf.clear();
        add_buf.clear();
    }

    fn build_unified_rows(
        diff: &FileDiff,
        old_highlights: &[Vec<HighlightRun>],
        new_highlights: &[Vec<HighlightRun>],
    ) -> Vec<DiffRow> {
        let mut rows = Vec::new();

        for hunk in &diff.hunks {
            for line in &hunk.lines {
                let hl = Self::get_highlights(line, old_highlights, new_highlights);
                rows.push(DiffRow {
                    left: Some(line.clone()),
                    right: None,
                    left_highlights: hl,
                    right_highlights: Vec::new(),
                });
            }
        }

        rows
    }

    pub fn toggle_stage_file(&mut self, idx: usize, cx: &mut Context<Self>) {
        let entry = match self.file_entries.get(idx) {
            Some(e) => e.clone(),
            None => return,
        };

        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        let path = entry.path.clone();
        let is_staged = entry.staged;

        cx.spawn(async move |this, cx| {
            let p = repo_path.clone();
            let file_path = path.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&p)?;
                if is_staged {
                    GitService::unstage_file(&repo, &file_path)
                } else {
                    GitService::stage_file(&repo, &file_path)
                }
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    if let Err(e) = result {
                        state.error_message = Some(format!("Stage/unstage failed: {}", e));
                    }
                    state.refresh(cx);
                });
            });
        })
        .detach();
    }

    pub fn do_commit(&mut self, cx: &mut Context<Self>) {
        let message = self.commit_editor.read(cx).content();
        let message = message.trim().to_string();
        if message.is_empty() {
            self.error_message = Some("Commit message cannot be empty".to_string());
            cx.notify();
            return;
        }

        let has_staged = self.file_entries.iter().any(|e| e.staged);
        if !has_staged {
            self.error_message = Some("No staged changes to commit".to_string());
            cx.notify();
            return;
        }

        let repo_path = match &self.repo_path {
            Some(p) => p.clone(),
            None => return,
        };

        self.loading = true;
        cx.notify();

        let commit_editor = self.commit_editor.clone();

        cx.spawn(async move |this, cx| {
            let p = repo_path.clone();
            let msg = message.clone();
            let result = smol::unblock(move || {
                let repo = GitService::open(&p)?;
                GitService::commit(&repo, &msg)
            })
            .await;

            let _ = cx.update(|cx| {
                let _ = this.update(cx, |state, cx| {
                    state.loading = false;
                    match result {
                        Ok(_oid) => {
                            state.error_message = None;
                            commit_editor.update(cx, |editor, cx| {
                                editor.set_content("", cx);
                            });
                            state.refresh(cx);
                        }
                        Err(e) => {
                            state.error_message = Some(format!("Commit failed: {}", e));
                            cx.notify();
                        }
                    }
                });
            });
        })
        .detach();
    }
}
