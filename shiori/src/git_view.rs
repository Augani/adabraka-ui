use crate::diff_highlighter::HighlightRun;
use crate::git_service::{DiffLineKind, FileStatusKind};
use crate::git_state::{DiffRow, DiffViewMode, GitState};
use adabraka_ui::components::button::{Button, ButtonVariant};
use adabraka_ui::components::editor::Editor;
use adabraka_ui::components::icon::Icon;
use adabraka_ui::components::resizable::{h_resizable, resizable_panel};
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use std::rc::Rc;

#[derive(Clone)]
struct DiffSplitDrag;

impl Render for DiffSplitDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(1.0))
            .h_full()
            .bg(gpui::hsla(0.58, 0.7, 0.6, 1.0))
    }
}

fn build_text_runs(
    content: &str,
    highlights: &[HighlightRun],
    default_color: Hsla,
) -> Vec<TextRun> {
    if content.is_empty() {
        return Vec::new();
    }

    let font = Font {
        family: "JetBrains Mono".into(),
        features: FontFeatures::default(),
        fallbacks: None,
        weight: FontWeight::NORMAL,
        style: FontStyle::Normal,
    };

    if highlights.is_empty() {
        return vec![TextRun {
            len: content.len(),
            font,
            color: default_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }];
    }

    let mut runs = Vec::new();
    let mut pos = 0;
    let content_len = content.len();

    for hl in highlights {
        if hl.start > content_len {
            break;
        }
        let hl_end = (hl.start + hl.len).min(content_len);
        if hl.start > pos {
            runs.push(TextRun {
                len: hl.start - pos,
                font: font.clone(),
                color: default_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
        }
        if hl_end > hl.start && hl.start >= pos {
            runs.push(TextRun {
                len: hl_end - hl.start,
                font: font.clone(),
                color: hl.color,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
            pos = hl_end;
        } else if hl.start < pos && hl_end > pos {
            runs.push(TextRun {
                len: hl_end - pos,
                font: font.clone(),
                color: hl.color,
                background_color: None,
                underline: None,
                strikethrough: None,
            });
            pos = hl_end;
        } else {
            pos = pos.max(hl_end);
        }
    }

    if pos < content_len {
        runs.push(TextRun {
            len: content_len - pos,
            font,
            color: default_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        });
    }

    runs
}

const LINE_HEIGHT: f32 = 20.0;
const GUTTER_WIDTH: f32 = 44.0;
const HEADER_HEIGHT: f32 = 32.0;

#[derive(IntoElement)]
pub struct GitView {
    state: Entity<GitState>,
}

impl GitView {
    pub fn new(state: Entity<GitState>) -> Self {
        Self { state }
    }

    fn render_file_entry(
        &self,
        idx: usize,
        entry_path: &str,
        entry_status: FileStatusKind,
        entry_staged: bool,
        entry_additions: usize,
        entry_deletions: usize,
        is_selected: bool,
    ) -> Stateful<Div> {
        let theme = use_theme();
        let green = gpui::hsla(0.33, 0.7, 0.5, 1.0);
        let red = theme.tokens.destructive;
        let yellow = gpui::hsla(0.12, 0.8, 0.55, 1.0);

        let (status_icon, icon_color) = if entry_staged {
            ("check", green)
        } else if entry_status == FileStatusKind::Untracked {
            ("file-plus", theme.tokens.muted_foreground)
        } else {
            ("circle-dot", yellow)
        };

        let filename = entry_path
            .rsplit('/')
            .next()
            .unwrap_or(entry_path)
            .to_string();
        let dir_part = if entry_path.contains('/') {
            let parts: Vec<&str> = entry_path.rsplitn(2, '/').collect();
            if parts.len() > 1 {
                Some(format!("{}/", parts[1]))
            } else {
                None
            }
        } else {
            None
        };

        let status_color = match entry_status {
            FileStatusKind::Added => green,
            FileStatusKind::Deleted => red,
            FileStatusKind::Modified => yellow,
            FileStatusKind::Renamed => theme.tokens.primary,
            FileStatusKind::Untracked => theme.tokens.muted_foreground,
        };

        let status_letter = match entry_status {
            FileStatusKind::Added => "A",
            FileStatusKind::Deleted => "D",
            FileStatusKind::Modified => "M",
            FileStatusKind::Renamed => "R",
            FileStatusKind::Untracked => "U",
        };

        let git_state_click = self.state.clone();
        let git_state_icon = self.state.clone();

        div()
            .id(ElementId::Name(format!("git-file-{}", idx).into()))
            .w_full()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(8.0))
            .py(px(3.0))
            .cursor_pointer()
            .text_size(px(13.0))
            .when(is_selected, |el| el.bg(theme.tokens.muted.opacity(0.5)))
            .hover(|s| s.bg(theme.tokens.muted.opacity(0.3)))
            .on_click(move |_, _, cx| {
                git_state_click.update(cx, |s, cx| s.select_file(idx, cx));
            })
            .child(
                div()
                    .id(ElementId::Name(format!("git-stage-icon-{}", idx).into()))
                    .w(px(18.0))
                    .h(px(18.0))
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded(px(3.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.tokens.muted.opacity(0.5)))
                    .on_click(move |_, _, cx| {
                        git_state_icon.update(cx, |s, cx| {
                            s.toggle_stage_file(idx, cx);
                        });
                    })
                    .child(Icon::new(status_icon).size(px(14.0)).color(icon_color)),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .overflow_x_hidden()
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .gap(px(2.0))
                            .overflow_x_hidden()
                            .text_ellipsis()
                            .children(dir_part.map(|d| {
                                div()
                                    .text_color(theme.tokens.muted_foreground)
                                    .text_size(px(12.0))
                                    .child(d)
                            }))
                            .child(div().text_color(theme.tokens.foreground).child(filename)),
                    )
                    .child(
                        div()
                            .flex_shrink_0()
                            .flex()
                            .gap(px(4.0))
                            .items_center()
                            .text_size(px(11.0))
                            .when(entry_additions > 0, |el| {
                                el.child(
                                    div()
                                        .text_color(green)
                                        .child(format!("+{}", entry_additions)),
                                )
                            })
                            .when(entry_deletions > 0, |el| {
                                el.child(
                                    div().text_color(red).child(format!("-{}", entry_deletions)),
                                )
                            })
                            .child(
                                div()
                                    .text_color(status_color)
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::BOLD)
                                    .child(status_letter),
                            ),
                    ),
            )
    }

    fn render_section_header(
        label: &str,
        count: usize,
        action_icon: &str,
        on_action: impl Fn(&mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        let theme = use_theme();
        div()
            .w_full()
            .h(px(26.0))
            .flex()
            .items_center()
            .justify_between()
            .px(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.tokens.muted_foreground)
                            .child(label.to_string()),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(theme.tokens.muted_foreground.opacity(0.6))
                            .px(px(4.0))
                            .h(px(16.0))
                            .flex()
                            .items_center()
                            .rounded(px(8.0))
                            .bg(theme.tokens.muted.opacity(0.3))
                            .child(format!("{}", count)),
                    ),
            )
            .child(
                div()
                    .id(ElementId::Name(format!("section-action-{}", label).into()))
                    .w(px(20.0))
                    .h(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(3.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.tokens.muted.opacity(0.5)))
                    .on_click(move |_, window, cx| on_action(window, cx))
                    .child(
                        Icon::new(action_icon)
                            .size(px(14.0))
                            .color(theme.tokens.muted_foreground),
                    ),
            )
    }

    fn render_file_list(&self, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let entries = &state.file_entries;
        let selected = state.selected_file_index;

        let staged_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.staged)
            .map(|(i, _)| i)
            .collect();

        let tracked_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.staged && e.status != FileStatusKind::Untracked)
            .map(|(i, _)| i)
            .collect();

        let untracked_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.staged && e.status == FileStatusKind::Untracked)
            .map(|(i, _)| i)
            .collect();

        let mut children: Vec<AnyElement> = Vec::new();

        if !staged_indices.is_empty() {
            let git_state = self.state.clone();
            children.push(
                Self::render_section_header(
                    "STAGED",
                    staged_indices.len(),
                    "minus",
                    move |_, cx| {
                        git_state.update(cx, |s, cx| s.unstage_all(cx));
                    },
                )
                .into_any_element(),
            );
            for &idx in &staged_indices {
                let e = &entries[idx];
                children.push(
                    self.render_file_entry(
                        idx,
                        &e.path,
                        e.status,
                        e.staged,
                        e.additions,
                        e.deletions,
                        idx == selected,
                    )
                    .into_any_element(),
                );
            }
        }

        if !tracked_indices.is_empty() {
            let git_state = self.state.clone();
            children.push(
                Self::render_section_header(
                    "CHANGES",
                    tracked_indices.len(),
                    "plus",
                    move |_, cx| {
                        git_state.update(cx, |s, cx| s.stage_tracked_changes(cx));
                    },
                )
                .into_any_element(),
            );
            for &idx in &tracked_indices {
                let e = &entries[idx];
                children.push(
                    self.render_file_entry(
                        idx,
                        &e.path,
                        e.status,
                        e.staged,
                        e.additions,
                        e.deletions,
                        idx == selected,
                    )
                    .into_any_element(),
                );
            }
        }

        if !untracked_indices.is_empty() {
            let git_state = self.state.clone();
            children.push(
                Self::render_section_header(
                    "UNTRACKED",
                    untracked_indices.len(),
                    "plus",
                    move |_, cx| {
                        git_state.update(cx, |s, cx| s.stage_untracked(cx));
                    },
                )
                .into_any_element(),
            );
            for &idx in &untracked_indices {
                let e = &entries[idx];
                children.push(
                    self.render_file_entry(
                        idx,
                        &e.path,
                        e.status,
                        e.staged,
                        e.additions,
                        e.deletions,
                        idx == selected,
                    )
                    .into_any_element(),
                );
            }
        }

        div()
            .id("git-file-list")
            .flex_1()
            .overflow_y_scroll()
            .children(children)
    }

    fn render_commit_area(&self, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let branch = state.summary.branch.clone();
        let has_staged = state.file_entries.iter().any(|e| e.staged);
        let loading = state.loading;
        let error = state.error_message.clone();
        let git_state = self.state.clone();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(px(8.0))
            .border_t_1()
            .border_color(theme.tokens.border)
            .child(
                div()
                    .w_full()
                    .h(px(80.0))
                    .border_1()
                    .border_color(theme.tokens.border)
                    .rounded(px(4.0))
                    .overflow_hidden()
                    .cursor_text()
                    .child(
                        Editor::new(&state.commit_editor)
                            .show_line_numbers(false, cx)
                            .show_border(false),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        Button::new("git-commit-btn", "Commit")
                            .variant(ButtonVariant::Default)
                            .disabled(!has_staged || loading)
                            .on_click(move |_, _, cx| {
                                git_state.update(cx, |s, cx| s.do_commit(cx));
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .child(
                                Icon::new("git-branch")
                                    .size(px(12.0))
                                    .color(theme.tokens.muted_foreground),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(branch),
                            ),
                    ),
            )
            .children(error.map(|msg| {
                div()
                    .w_full()
                    .text_size(px(11.0))
                    .text_color(theme.tokens.destructive)
                    .child(msg)
            }))
    }

    fn render_new_file_diff(rows: Rc<Vec<DiffRow>>) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-newfile",
            item_count,
            move |range, _window, _cx| {
                let theme = use_theme();
                let green_bg = gpui::hsla(0.33, 0.7, 0.5, 0.15);
                let default_color = theme.tokens.foreground;
                let muted_fg = theme.tokens.muted_foreground.opacity(0.5);

                range
                    .map(|row_idx| {
                        let row = &rows[row_idx];
                        let line = row.right.as_ref().or(row.left.as_ref());
                        let line = match line {
                            Some(l) => l,
                            None => return div().h(line_h).into_any_element(),
                        };

                        let lineno = line
                            .new_lineno
                            .or(line.old_lineno)
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();

                        let content = line.content.clone();
                        let highlights = if row.right.is_some() {
                            &row.right_highlights
                        } else {
                            &row.left_highlights
                        };

                        let styled = if !content.is_empty() {
                            let text_runs = build_text_runs(&content, highlights, default_color);
                            StyledText::new(SharedString::from(content))
                                .with_runs(text_runs)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        };

                        div()
                            .w_full()
                            .h(line_h)
                            .flex()
                            .overflow_x_hidden()
                            .bg(green_bg)
                            .child(
                                div()
                                    .w(gutter_w)
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_end()
                                    .px(px(4.0))
                                    .text_size(px(11.0))
                                    .text_color(muted_fg)
                                    .child(lineno),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .px(px(8.0))
                                    .child(styled),
                            )
                            .into_any_element()
                    })
                    .collect()
            },
        )
        .h_full()
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_split_diff(
        rows: Rc<Vec<DiffRow>>,
        git_state: Entity<GitState>,
        split_pct: f32,
    ) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);
        let split = split_pct;

        let list = uniform_list(
            "diff-scroll-split",
            item_count,
            move |range, _window, _cx| {
                let theme = use_theme();
                let green_bg = gpui::hsla(0.33, 0.7, 0.5, 0.15);
                let red_bg = gpui::hsla(0.0, 0.7, 0.5, 0.15);
                let filler_bg = theme.tokens.muted.opacity(0.05);
                let default_color = theme.tokens.foreground;
                let muted_fg = theme.tokens.muted_foreground.opacity(0.5);
                let border_color = theme.tokens.border.opacity(0.3);

                range
                    .map(|row_idx| {
                        let row = &rows[row_idx];

                        let left_bg = match &row.left {
                            Some(l) => match l.kind {
                                DiffLineKind::Deletion => red_bg,
                                DiffLineKind::Context => gpui::hsla(0.0, 0.0, 0.0, 0.0),
                                DiffLineKind::Addition => green_bg,
                            },
                            None => filler_bg,
                        };

                        let right_bg = match &row.right {
                            Some(r) => match r.kind {
                                DiffLineKind::Addition => green_bg,
                                DiffLineKind::Context => gpui::hsla(0.0, 0.0, 0.0, 0.0),
                                DiffLineKind::Deletion => red_bg,
                            },
                            None => filler_bg,
                        };

                        let left_lineno = row
                            .left
                            .as_ref()
                            .and_then(|l| l.old_lineno)
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();
                        let left_content = row
                            .left
                            .as_ref()
                            .map(|l| l.content.clone())
                            .unwrap_or_default();

                        let right_lineno = row
                            .right
                            .as_ref()
                            .and_then(|r| r.new_lineno)
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();
                        let right_content = row
                            .right
                            .as_ref()
                            .map(|r| r.content.clone())
                            .unwrap_or_default();

                        let left_styled = if !left_content.is_empty() {
                            let text_runs =
                                build_text_runs(&left_content, &row.left_highlights, default_color);
                            StyledText::new(SharedString::from(left_content))
                                .with_runs(text_runs)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        };

                        let right_styled = if !right_content.is_empty() {
                            let text_runs = build_text_runs(
                                &right_content,
                                &row.right_highlights,
                                default_color,
                            );
                            StyledText::new(SharedString::from(right_content))
                                .with_runs(text_runs)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        };

                        div()
                            .w_full()
                            .h(line_h)
                            .flex()
                            .child(
                                div()
                                    .w(relative(split))
                                    .h(line_h)
                                    .flex()
                                    .overflow_x_hidden()
                                    .bg(left_bg)
                                    .child(
                                        div()
                                            .w(gutter_w)
                                            .h_full()
                                            .flex()
                                            .flex_shrink_0()
                                            .items_center()
                                            .justify_end()
                                            .px(px(4.0))
                                            .text_size(px(11.0))
                                            .text_color(muted_fg)
                                            .child(left_lineno),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .px(px(8.0))
                                            .child(left_styled),
                                    ),
                            )
                            .child(div().w(px(1.0)).h(line_h).bg(border_color))
                            .child(
                                div()
                                    .flex_1()
                                    .h(line_h)
                                    .flex()
                                    .overflow_x_hidden()
                                    .bg(right_bg)
                                    .child(
                                        div()
                                            .w(gutter_w)
                                            .h_full()
                                            .flex()
                                            .flex_shrink_0()
                                            .items_center()
                                            .justify_end()
                                            .px(px(4.0))
                                            .text_size(px(11.0))
                                            .text_color(muted_fg)
                                            .child(right_lineno),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .px(px(8.0))
                                            .child(right_styled),
                                    ),
                            )
                            .into_any_element()
                    })
                    .collect()
            },
        )
        .h_full()
        .font_family("JetBrains Mono")
        .text_size(px(13.0));

        let theme = use_theme();
        let state_for_drag = git_state.clone();

        div()
            .id("diff-split-container")
            .size_full()
            .relative()
            .child(list)
            .child(
                div()
                    .id("diff-split-handle")
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(relative(split_pct))
                    .w(px(9.0))
                    .ml(px(-4.0))
                    .cursor_col_resize()
                    .child(
                        div()
                            .w(px(1.0))
                            .h_full()
                            .mx_auto()
                            .bg(theme.tokens.border.opacity(0.5))
                            .hover(|s| s.bg(theme.tokens.accent)),
                    )
                    .on_drag(
                        DiffSplitDrag,
                        |_drag: &DiffSplitDrag, _position, _window, cx| cx.new(|_| DiffSplitDrag),
                    ),
            )
            .on_drag_move::<DiffSplitDrag>(move |event, _window, cx| {
                let mouse_x = event.event.position.x;
                let bounds = event.bounds;
                if bounds.size.width > px(0.0) {
                    let new_pct = (mouse_x - bounds.origin.x) / bounds.size.width;
                    let clamped = new_pct.clamp(0.2, 0.8);
                    state_for_drag.update(cx, |s, cx| {
                        s.set_diff_split_pct(clamped, cx);
                    });
                }
            })
    }

    fn render_unified_diff(rows: Rc<Vec<DiffRow>>) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-unified",
            item_count,
            move |range, _window, _cx| {
                let theme = use_theme();
                let green_bg = gpui::hsla(0.33, 0.7, 0.5, 0.15);
                let red_bg = gpui::hsla(0.0, 0.7, 0.5, 0.15);
                let default_color = theme.tokens.foreground;
                let muted_fg = theme.tokens.muted_foreground.opacity(0.5);

                range
                    .map(|row_idx| {
                        let row = &rows[row_idx];

                        let line = match &row.left {
                            Some(l) => l,
                            None => {
                                return div().h(line_h).into_any_element();
                            }
                        };

                        let (bg, prefix) = match line.kind {
                            DiffLineKind::Addition => (green_bg, "+"),
                            DiffLineKind::Deletion => (red_bg, "-"),
                            DiffLineKind::Context => (gpui::hsla(0.0, 0.0, 0.0, 0.0), " "),
                        };

                        let old_no = line
                            .old_lineno
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();
                        let new_no = line
                            .new_lineno
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();

                        let content = line.content.clone();
                        let styled_content = if !content.is_empty() {
                            let text_runs =
                                build_text_runs(&content, &row.left_highlights, default_color);
                            StyledText::new(SharedString::from(content))
                                .with_runs(text_runs)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        };

                        div()
                            .w_full()
                            .h(line_h)
                            .flex()
                            .overflow_x_hidden()
                            .bg(bg)
                            .child(
                                div()
                                    .w(gutter_w)
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_end()
                                    .px(px(4.0))
                                    .text_size(px(11.0))
                                    .text_color(muted_fg)
                                    .child(old_no),
                            )
                            .child(
                                div()
                                    .w(gutter_w)
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_end()
                                    .px(px(4.0))
                                    .text_size(px(11.0))
                                    .text_color(muted_fg)
                                    .child(new_no),
                            )
                            .child(
                                div()
                                    .w(px(20.0))
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child(prefix),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .px(px(8.0))
                                    .child(styled_content),
                            )
                            .into_any_element()
                    })
                    .collect()
            },
        )
        .h_full()
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_deleted_file_diff(rows: Rc<Vec<DiffRow>>) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-deleted",
            item_count,
            move |range, _window, _cx| {
                let theme = use_theme();
                let red_bg = gpui::hsla(0.0, 0.7, 0.5, 0.15);
                let default_color = theme.tokens.foreground;
                let muted_fg = theme.tokens.muted_foreground.opacity(0.5);

                range
                    .map(|row_idx| {
                        let row = &rows[row_idx];
                        let line = row.left.as_ref().or(row.right.as_ref());
                        let line = match line {
                            Some(l) => l,
                            None => return div().h(line_h).into_any_element(),
                        };

                        let lineno = line
                            .old_lineno
                            .or(line.new_lineno)
                            .map(|n| format!("{}", n))
                            .unwrap_or_default();

                        let content = line.content.clone();
                        let highlights = if row.left.is_some() {
                            &row.left_highlights
                        } else {
                            &row.right_highlights
                        };

                        let styled = if !content.is_empty() {
                            let text_runs = build_text_runs(&content, highlights, default_color);
                            StyledText::new(SharedString::from(content))
                                .with_runs(text_runs)
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        };

                        div()
                            .w_full()
                            .h(line_h)
                            .flex()
                            .overflow_x_hidden()
                            .bg(red_bg)
                            .child(
                                div()
                                    .w(gutter_w)
                                    .h_full()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_end()
                                    .px(px(4.0))
                                    .text_size(px(11.0))
                                    .text_color(muted_fg)
                                    .child(lineno),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .px(px(8.0))
                                    .child(styled),
                            )
                            .into_any_element()
                    })
                    .collect()
            },
        )
        .h_full()
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_diff_panel(&self, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let header_h = px(HEADER_HEIGHT);

        let (is_empty, has_diff, is_binary, diff_path, rows, view_mode, file_status, split_pct) = {
            let state = self.state.read(cx);
            let is_empty = state.file_entries.is_empty();
            let has_diff = state.active_diff.is_some();
            let is_binary = state
                .active_diff
                .as_ref()
                .map(|d| d.is_binary)
                .unwrap_or(false);
            let diff_path = state
                .active_diff
                .as_ref()
                .map(|d| d.path.clone())
                .unwrap_or_default();
            let rows = Rc::new(state.aligned_rows.clone());
            let view_mode = state.diff_view_mode;
            let file_status = state
                .file_entries
                .get(state.selected_file_index)
                .map(|e| e.status);
            let split_pct = state.diff_split_pct;
            (
                is_empty,
                has_diff,
                is_binary,
                diff_path,
                rows,
                view_mode,
                file_status,
                split_pct,
            )
        };

        let is_new_file = matches!(
            file_status,
            Some(FileStatusKind::Added) | Some(FileStatusKind::Untracked)
        );

        let is_deleted_file = matches!(file_status, Some(FileStatusKind::Deleted));

        if is_empty {
            return div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .h(header_h)
                        .border_b_1()
                        .border_color(theme.tokens.border),
                )
                .child(
                    div().flex_1().flex().items_center().justify_center().child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                Icon::new("circle-check")
                                    .size(px(32.0))
                                    .color(theme.tokens.muted_foreground.opacity(0.3)),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(theme.tokens.muted_foreground)
                                    .child("Working tree clean"),
                            ),
                    ),
                )
                .into_any_element();
        }

        if !has_diff {
            return div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .h(header_h)
                        .border_b_1()
                        .border_color(theme.tokens.border),
                )
                .child(
                    div().flex_1().flex().items_center().justify_center().child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Select a file to view diff"),
                    ),
                )
                .into_any_element();
        }

        if is_binary {
            return div()
                .size_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .w_full()
                        .h(header_h)
                        .border_b_1()
                        .border_color(theme.tokens.border),
                )
                .child(
                    div().flex_1().flex().items_center().justify_center().child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.tokens.muted_foreground)
                            .child("Binary file"),
                    ),
                )
                .into_any_element();
        }

        let single_pane = is_new_file || is_deleted_file;

        let git_state_split = self.state.clone();
        let git_state_unified = self.state.clone();

        let split_active = view_mode == DiffViewMode::Split;
        let unified_active = view_mode == DiffViewMode::Unified;

        let status_label = if is_new_file {
            Some("New file")
        } else if is_deleted_file {
            Some("Deleted")
        } else {
            None
        };

        let green = gpui::hsla(0.33, 0.7, 0.5, 1.0);

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .h(header_h)
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .border_b_1()
                    .border_color(theme.tokens.border)
                    .bg(theme.tokens.muted.opacity(0.2))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(theme.tokens.foreground)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(diff_path),
                            )
                            .children(status_label.map(|label| {
                                div()
                                    .px(px(6.0))
                                    .py(px(1.0))
                                    .rounded(px(3.0))
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .when(is_new_file, |el| {
                                        el.bg(green.opacity(0.15)).text_color(green)
                                    })
                                    .when(is_deleted_file, |el| {
                                        el.bg(theme.tokens.destructive.opacity(0.15))
                                            .text_color(theme.tokens.destructive)
                                    })
                                    .child(label)
                            })),
                    )
                    .when(!single_pane, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .id("view-split-btn")
                                        .px(px(6.0))
                                        .h(px(22.0))
                                        .flex()
                                        .items_center()
                                        .rounded(px(3.0))
                                        .cursor_pointer()
                                        .text_size(px(11.0))
                                        .when(split_active, |el| {
                                            el.bg(theme.tokens.muted.opacity(0.6))
                                                .text_color(theme.tokens.foreground)
                                        })
                                        .when(!split_active, |el| {
                                            el.text_color(theme.tokens.muted_foreground)
                                                .hover(|s| s.bg(theme.tokens.muted.opacity(0.3)))
                                        })
                                        .on_click(move |_, _, cx| {
                                            git_state_split.update(cx, |s, cx| {
                                                s.set_diff_view_mode(DiffViewMode::Split, cx);
                                            });
                                        })
                                        .child(Icon::new("columns-2").size(px(14.0)).color(
                                            if split_active {
                                                theme.tokens.foreground
                                            } else {
                                                theme.tokens.muted_foreground
                                            },
                                        )),
                                )
                                .child(
                                    div()
                                        .id("view-unified-btn")
                                        .px(px(6.0))
                                        .h(px(22.0))
                                        .flex()
                                        .items_center()
                                        .rounded(px(3.0))
                                        .cursor_pointer()
                                        .text_size(px(11.0))
                                        .when(unified_active, |el| {
                                            el.bg(theme.tokens.muted.opacity(0.6))
                                                .text_color(theme.tokens.foreground)
                                        })
                                        .when(!unified_active, |el| {
                                            el.text_color(theme.tokens.muted_foreground)
                                                .hover(|s| s.bg(theme.tokens.muted.opacity(0.3)))
                                        })
                                        .on_click(move |_, _, cx| {
                                            git_state_unified.update(cx, |s, cx| {
                                                s.set_diff_view_mode(DiffViewMode::Unified, cx);
                                            });
                                        })
                                        .child(Icon::new("rows-2").size(px(14.0)).color(
                                            if unified_active {
                                                theme.tokens.foreground
                                            } else {
                                                theme.tokens.muted_foreground
                                            },
                                        )),
                                ),
                        )
                    }),
            )
            .child(div().flex_1().overflow_hidden().child(if is_new_file {
                Self::render_new_file_diff(rows).into_any_element()
            } else if is_deleted_file {
                Self::render_deleted_file_diff(rows).into_any_element()
            } else {
                match view_mode {
                    DiffViewMode::Split => {
                        Self::render_split_diff(rows, self.state.clone(), split_pct)
                            .into_any_element()
                    }
                    DiffViewMode::Unified => Self::render_unified_diff(rows).into_any_element(),
                }
            }))
            .into_any_element()
    }
}

impl RenderOnce for GitView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = use_theme();
        let panel_resizable = self.state.read(cx).panel_resizable.clone();
        let header_h = px(HEADER_HEIGHT);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.tokens.background)
            .child(
                div().flex_1().overflow_hidden().child(
                    h_resizable("git-layout", panel_resizable)
                        .child(
                            resizable_panel()
                                .size(px(280.0))
                                .min_size(px(180.0))
                                .max_size(px(500.0))
                                .child(
                                    div()
                                        .size_full()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .w_full()
                                                .h(header_h)
                                                .flex()
                                                .items_center()
                                                .px(px(12.0))
                                                .border_b_1()
                                                .border_color(theme.tokens.border)
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.tokens.muted_foreground)
                                                        .child("SOURCE CONTROL"),
                                                ),
                                        )
                                        .child(self.render_file_list(cx))
                                        .child(self.render_commit_area(cx)),
                                ),
                        )
                        .child(resizable_panel().child(self.render_diff_panel(cx))),
                ),
            )
    }
}
