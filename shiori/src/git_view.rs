use crate::diff_highlighter::HighlightRun;
use crate::git_service::{DiffLineKind, FileStatusKind};
use crate::git_state::{DiffRow, DiffViewMode, GitState};
use crate::ide_theme::use_ide_theme;
use adabraka_ui::components::icon::Icon;
use adabraka_ui::theme::use_theme;
use gpui::prelude::FluentBuilder as _;
use gpui::UniformListScrollHandle;
use gpui::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct ScrollbarThumbDrag<T: 'static> {
    pub scroll_handle: ScrollHandle,
    pub notifier: Entity<T>,
    pub total_content_h: f32,
}

impl<T: 'static> Render for ScrollbarThumbDrag<T> {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().w(px(0.0)).h(px(0.0)).overflow_hidden()
    }
}

pub fn render_vertical_scrollbar<T: 'static>(
    id: impl Into<SharedString>,
    scroll_handle: ScrollHandle,
    total_content_h: f32,
    notifier: Entity<T>,
) -> AnyElement {
    let id: SharedString = id.into();
    let theme = use_theme();
    let viewport_h = scroll_handle.bounds().size.height;
    let total_h = px(total_content_h);

    if viewport_h <= px(0.0) || total_h <= viewport_h {
        return div().into_any_element();
    }

    let visible_ratio = (viewport_h / total_h).min(1.0);
    let thumb_h_pct = (visible_ratio * 100.0).max(5.0);
    let scroll_offset = -scroll_handle.offset().y;
    let scroll_range = total_h - viewport_h;
    let thumb_top_pct = if scroll_range > px(0.0) {
        ((scroll_offset / scroll_range) * (100.0 - thumb_h_pct)).max(0.0)
    } else {
        0.0
    };

    let handle_for_click = scroll_handle.clone();
    let state_for_click = notifier.clone();

    let handle_for_drag = scroll_handle.clone();
    let state_for_drag = notifier.clone();
    let total_h_f32 = total_content_h;

    let thumb_id: SharedString = format!("{}-thumb", id).into();

    div()
        .id(ElementId::Name(id.into()))
        .absolute()
        .top_0()
        .right_0()
        .bottom_0()
        .w(px(12.0))
        .cursor(CursorStyle::PointingHand)
        .on_mouse_down(
            MouseButton::Left,
            move |event: &MouseDownEvent, _window, cx| {
                cx.stop_propagation();
                let bar_top = handle_for_click.bounds().origin.y;
                let bar_h = handle_for_click.bounds().size.height;
                if bar_h <= px(0.0) {
                    return;
                }
                let click_ratio = ((event.position.y - bar_top) / bar_h).max(0.0).min(1.0);
                let max_scroll = total_h - viewport_h;
                let new_offset_y = -(max_scroll * click_ratio);
                handle_for_click.set_offset(point(handle_for_click.offset().x, new_offset_y));
                state_for_click.update(cx, |_, cx| cx.notify());
            },
        )
        .on_drag_move::<ScrollbarThumbDrag<T>>(move |event, _window, cx| {
            let bounds = event.bounds;
            let mouse_y = event.event.position.y;
            let bar_h = bounds.size.height;
            if bar_h <= px(0.0) {
                return;
            }
            let ratio = ((mouse_y - bounds.origin.y) / bar_h).max(0.0).min(1.0);
            let vp_h = handle_for_drag.bounds().size.height;
            let max_scroll = px(total_h_f32) - vp_h;
            if max_scroll > px(0.0) {
                let new_offset_y = -(max_scroll * ratio);
                handle_for_drag.set_offset(point(handle_for_drag.offset().x, new_offset_y));
                state_for_drag.update(cx, |_, cx| cx.notify());
            }
        })
        .child(
            div()
                .id(ElementId::Name(thumb_id))
                .absolute()
                .left(px(2.0))
                .right(px(2.0))
                .top(relative(thumb_top_pct / 100.0))
                .h(relative(thumb_h_pct / 100.0))
                .bg(theme.tokens.muted_foreground.opacity(0.4))
                .rounded(px(3.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(theme.tokens.muted_foreground.opacity(0.7)))
                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                    cx.stop_propagation();
                })
                .on_drag(
                    ScrollbarThumbDrag {
                        scroll_handle: scroll_handle.clone(),
                        notifier: notifier.clone(),
                        total_content_h,
                    },
                    |drag: &ScrollbarThumbDrag<T>, _, _, cx| {
                        cx.new(|_| ScrollbarThumbDrag {
                            scroll_handle: drag.scroll_handle.clone(),
                            notifier: drag.notifier.clone(),
                            total_content_h: drag.total_content_h,
                        })
                    },
                ),
        )
        .into_any_element()
}

#[derive(Clone)]
struct DiffSplitDrag;

impl Render for DiffSplitDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let chrome = use_ide_theme().chrome;
        div().w(px(1.0)).h_full().bg(chrome.accent)
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

    fn render_new_file_diff(
        rows: Rc<Vec<DiffRow>>,
        scroll_handle: UniformListScrollHandle,
    ) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-newfile",
            item_count,
            move |range, _window, _cx| {
                let chrome = use_ide_theme().chrome;
                let green_bg = chrome.diff_add_bg;
                let default_color = chrome.bright;
                let muted_fg = chrome.text_secondary.opacity(0.5);

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
        .track_scroll(scroll_handle)
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_split_diff(
        rows: Rc<Vec<DiffRow>>,
        git_state: Entity<GitState>,
        split_pct: f32,
        scroll_handle: UniformListScrollHandle,
    ) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);
        let split = split_pct;

        let list = uniform_list(
            "diff-scroll-split",
            item_count,
            move |range, _window, _cx| {
                let chrome = use_ide_theme().chrome;
                let green_bg = chrome.diff_add_bg;
                let red_bg = chrome.diff_del_bg;
                let filler_bg = chrome.dim.opacity(0.05);
                let default_color = chrome.bright;
                let muted_fg = chrome.text_secondary.opacity(0.5);
                let border_color = chrome.header_border.opacity(0.3);

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
        .track_scroll(scroll_handle)
        .font_family("JetBrains Mono")
        .text_size(px(13.0));

        let chrome = use_ide_theme().chrome;
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
                            .bg(chrome.header_border.opacity(0.5))
                            .hover(|s| s.bg(chrome.accent)),
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

    fn render_unified_diff(
        rows: Rc<Vec<DiffRow>>,
        scroll_handle: UniformListScrollHandle,
    ) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-unified",
            item_count,
            move |range, _window, _cx| {
                let chrome = use_ide_theme().chrome;
                let green_bg = chrome.diff_add_bg;
                let red_bg = chrome.diff_del_bg;
                let default_color = chrome.bright;
                let muted_fg = chrome.text_secondary.opacity(0.5);

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
                                    .text_color(chrome.text_secondary)
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
        .track_scroll(scroll_handle)
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_deleted_file_diff(
        rows: Rc<Vec<DiffRow>>,
        scroll_handle: UniformListScrollHandle,
    ) -> impl IntoElement {
        let item_count = rows.len();
        let line_h = px(LINE_HEIGHT);
        let gutter_w = px(GUTTER_WIDTH);

        uniform_list(
            "diff-scroll-deleted",
            item_count,
            move |range, _window, _cx| {
                let chrome = use_ide_theme().chrome;
                let red_bg = chrome.diff_del_bg;
                let default_color = chrome.bright;
                let muted_fg = chrome.text_secondary.opacity(0.5);

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
        .track_scroll(scroll_handle)
        .font_family("JetBrains Mono")
        .text_size(px(13.0))
    }

    fn render_diff_panel(&self, cx: &mut App) -> impl IntoElement {
        let chrome = use_ide_theme().chrome;
        let header_h = px(HEADER_HEIGHT);

        let (
            is_empty,
            has_diff,
            is_binary,
            diff_path,
            rows,
            view_mode,
            file_status,
            split_pct,
            scroll_handle,
        ) = {
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
            let scroll_handle = state.diff_scroll_handle.clone();
            (
                is_empty,
                has_diff,
                is_binary,
                diff_path,
                rows,
                view_mode,
                file_status,
                split_pct,
                scroll_handle,
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
                        .border_color(chrome.header_border),
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
                                    .color(chrome.text_secondary.opacity(0.3)),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(chrome.text_secondary)
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
                        .border_color(chrome.header_border),
                )
                .child(
                    div().flex_1().flex().items_center().justify_center().child(
                        div()
                            .text_size(px(13.0))
                            .text_color(chrome.text_secondary)
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
                        .border_color(chrome.header_border),
                )
                .child(
                    div().flex_1().flex().items_center().justify_center().child(
                        div()
                            .text_size(px(13.0))
                            .text_color(chrome.text_secondary)
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

        let green = chrome.diff_add_text;
        let del_color = chrome.diff_del_text;

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
                    .border_color(chrome.header_border)
                    .bg(chrome.dim.opacity(0.2))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(chrome.bright)
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
                                        el.bg(del_color.opacity(0.15)).text_color(del_color)
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
                                            el.bg(chrome.dim.opacity(0.6)).text_color(chrome.bright)
                                        })
                                        .when(!split_active, |el| {
                                            el.text_color(chrome.text_secondary)
                                                .hover(|s| s.bg(chrome.dim.opacity(0.3)))
                                        })
                                        .on_click(move |_, _, cx| {
                                            git_state_split.update(cx, |s, cx| {
                                                s.set_diff_view_mode(DiffViewMode::Split, cx);
                                            });
                                        })
                                        .child(Icon::new("columns-2").size(px(14.0)).color(
                                            if split_active {
                                                chrome.bright
                                            } else {
                                                chrome.text_secondary
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
                                            el.bg(chrome.dim.opacity(0.6)).text_color(chrome.bright)
                                        })
                                        .when(!unified_active, |el| {
                                            el.text_color(chrome.text_secondary)
                                                .hover(|s| s.bg(chrome.dim.opacity(0.3)))
                                        })
                                        .on_click(move |_, _, cx| {
                                            git_state_unified.update(cx, |s, cx| {
                                                s.set_diff_view_mode(DiffViewMode::Unified, cx);
                                            });
                                        })
                                        .child(Icon::new("rows-2").size(px(14.0)).color(
                                            if unified_active {
                                                chrome.bright
                                            } else {
                                                chrome.text_secondary
                                            },
                                        )),
                                ),
                        )
                    }),
            )
            .child({
                let row_count = rows.len();
                let git_state_bar = self.state.clone();
                div()
                    .id("diff-scroll-container")
                    .flex_1()
                    .overflow_hidden()
                    .relative()
                    .child(if is_new_file {
                        Self::render_new_file_diff(rows, scroll_handle.clone()).into_any_element()
                    } else if is_deleted_file {
                        Self::render_deleted_file_diff(rows, scroll_handle.clone())
                            .into_any_element()
                    } else {
                        match view_mode {
                            DiffViewMode::Split => Self::render_split_diff(
                                rows,
                                self.state.clone(),
                                split_pct,
                                scroll_handle.clone(),
                            )
                            .into_any_element(),
                            DiffViewMode::Unified => {
                                Self::render_unified_diff(rows, scroll_handle.clone())
                                    .into_any_element()
                            }
                        }
                    })
                    .child({
                        let base_handle = scroll_handle.0.borrow().base_handle.clone();
                        let total_h = row_count as f32 * LINE_HEIGHT;
                        render_vertical_scrollbar(
                            "diff-vscroll",
                            base_handle,
                            total_h,
                            git_state_bar,
                        )
                    })
            })
            .into_any_element()
    }
}

impl RenderOnce for GitView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_diff_panel(cx))
    }
}
