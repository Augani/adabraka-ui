//! ImageViewer/Lightbox component for displaying images in a fullscreen overlay.

use gpui::{prelude::FluentBuilder as _, *};
use std::rc::Rc;

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::use_theme;

actions!(
    image_viewer,
    [
        ImageViewerClose,
        ImageViewerNext,
        ImageViewerPrev,
        ImageViewerZoomIn,
        ImageViewerZoomOut,
        ImageViewerResetZoom
    ]
);

#[derive(Clone)]
pub struct ImageItem {
    pub src: SharedString,
    pub alt: Option<SharedString>,
    pub caption: Option<SharedString>,
}

impl ImageItem {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            alt: None,
            caption: None,
        }
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageViewerSize {
    Auto,
    Contain,
    Cover,
    Custom(f32),
}

impl Default for ImageViewerSize {
    fn default() -> Self {
        Self::Contain
    }
}

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;
const ZOOM_STEP: f32 = 0.25;

pub struct ImageViewerState {
    images: Vec<ImageItem>,
    current_index: usize,
    zoom: f32,
    pan_offset: Point<Pixels>,
    _is_panning: bool,
    _last_mouse_pos: Point<Pixels>,
    _loading: bool,
    show_thumbnails: bool,
    _fit_mode: ImageViewerSize,
}

impl ImageViewerState {
    pub fn new(images: Vec<ImageItem>) -> Self {
        Self {
            images,
            current_index: 0,
            zoom: 1.0,
            pan_offset: point(px(0.0), px(0.0)),
            _is_panning: false,
            _last_mouse_pos: point(px(0.0), px(0.0)),
            _loading: false,
            show_thumbnails: true,
            _fit_mode: ImageViewerSize::default(),
        }
    }

    pub fn set_images(&mut self, images: Vec<ImageItem>) {
        self.images = images;
        self.current_index = 0;
        self.reset_view();
    }

    pub fn go_to(&mut self, index: usize) {
        if index < self.images.len() {
            self.current_index = index;
            self.reset_view();
        }
    }

    pub fn next(&mut self) {
        if self.current_index < self.images.len().saturating_sub(1) {
            self.current_index += 1;
            self.reset_view();
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.reset_view();
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + ZOOM_STEP).min(MAX_ZOOM);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - ZOOM_STEP).max(MIN_ZOOM);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
        self.pan_offset = point(px(0.0), px(0.0));
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    }

    pub fn toggle_thumbnails(&mut self) {
        self.show_thumbnails = !self.show_thumbnails;
    }

    fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_offset = point(px(0.0), px(0.0));
    }

    pub fn current_image(&self) -> Option<&ImageItem> {
        self.images.get(self.current_index)
    }

    pub fn has_next(&self) -> bool {
        self.current_index < self.images.len().saturating_sub(1)
    }

    pub fn has_prev(&self) -> bool {
        self.current_index > 0
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn is_zoomed(&self) -> bool {
        (self.zoom - 1.0).abs() > 0.01
    }
}

pub struct ImageViewer {
    focus_handle: FocusHandle,
    state: Entity<ImageViewerState>,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    close_on_backdrop_click: bool,
    close_on_escape: bool,
    show_controls: bool,
    show_counter: bool,
    show_thumbnails: bool,
    style: StyleRefinement,
}

impl ImageViewer {
    pub fn new(state: Entity<ImageViewerState>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            state,
            on_close: None,
            close_on_backdrop_click: true,
            close_on_escape: true,
            show_controls: true,
            show_counter: true,
            show_thumbnails: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn close_on_backdrop_click(mut self, close: bool) -> Self {
        self.close_on_backdrop_click = close;
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn show_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    pub fn show_counter(mut self, show: bool) -> Self {
        self.show_counter = show;
        self
    }

    pub fn show_thumbnails(mut self, show: bool) -> Self {
        self.show_thumbnails = show;
        self
    }

    fn handle_close(&self, window: &mut Window, cx: &mut App) {
        if let Some(handler) = &self.on_close {
            (handler)(window, cx);
        }
    }
}

impl Styled for ImageViewer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Focusable for ImageViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for ImageViewer {}

impl Render for ImageViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();
        let state = self.state.read(cx);
        let current_image = state.current_image().cloned();
        let current_index = state.current_index();
        let image_count = state.image_count();
        let zoom = state.zoom();
        let has_prev = state.has_prev();
        let has_next = state.has_next();
        let images = state.images.clone();
        let _pan_offset = state.pan_offset;
        let show_thumbs = self.show_thumbnails && state.show_thumbnails && image_count > 1;

        let viewer_entity = cx.entity().clone();
        let state_entity = self.state.clone();

        window.focus(&self.focus_handle);

        let close_handler = self.on_close.clone();
        let close_on_escape = self.close_on_escape;
        let close_on_backdrop = self.close_on_backdrop_click;

        div()
            .id("image-viewer-overlay")
            .key_context("ImageViewer")
            .track_focus(&self.focus_handle)
            .absolute()
            .inset_0()
            .flex()
            .flex_col()
            .bg(gpui::black().opacity(0.9))
            .on_action({
                let viewer_entity = viewer_entity.clone();
                move |_: &ImageViewerClose, window, cx| {
                    if close_on_escape {
                        cx.update_entity(&viewer_entity, |viewer, cx| {
                            viewer.handle_close(window, cx);
                        });
                    }
                }
            })
            .on_action({
                let state_entity = state_entity.clone();
                move |_: &ImageViewerNext, _, cx| {
                    cx.update_entity(&state_entity, |state, _| state.next());
                }
            })
            .on_action({
                let state_entity = state_entity.clone();
                move |_: &ImageViewerPrev, _, cx| {
                    cx.update_entity(&state_entity, |state, _| state.prev());
                }
            })
            .on_action({
                let state_entity = state_entity.clone();
                move |_: &ImageViewerZoomIn, _, cx| {
                    cx.update_entity(&state_entity, |state, _| state.zoom_in());
                }
            })
            .on_action({
                let state_entity = state_entity.clone();
                move |_: &ImageViewerZoomOut, _, cx| {
                    cx.update_entity(&state_entity, |state, _| state.zoom_out());
                }
            })
            .on_action({
                let state_entity = state_entity.clone();
                move |_: &ImageViewerResetZoom, _, cx| {
                    cx.update_entity(&state_entity, |state, _| state.reset_zoom());
                }
            })
            .child(
                div()
                    .id("image-viewer-header")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(16.0))
                    .py(px(12.0))
                    .child(div().flex().items_center().gap(px(8.0)).when(
                        self.show_counter && image_count > 1,
                        |this| {
                            this.child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(gpui::white())
                                    .font_family(theme.tokens.font_family.clone())
                                    .child(format!("{} of {}", current_index + 1, image_count)),
                            )
                        },
                    ))
                    .child(div().flex().items_center().gap(px(4.0)).when(
                        self.show_controls,
                        |this| {
                            let state_entity = state_entity.clone();
                            let state_entity2 = state_entity.clone();
                            let state_entity3 = state_entity.clone();

                            this.child(
                                Button::new("zoom-out", "")
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Icon)
                                    .icon("zoom-out")
                                    .on_click(move |_, _, cx| {
                                        cx.update_entity(&state_entity, |state, _| {
                                            state.zoom_out()
                                        });
                                    }),
                            )
                            .child(
                                div()
                                    .min_w(px(50.0))
                                    .text_size(px(13.0))
                                    .text_color(gpui::white())
                                    .font_family(theme.tokens.font_family.clone())
                                    .text_center()
                                    .child(format!("{}%", (zoom * 100.0) as i32)),
                            )
                            .child(
                                Button::new("zoom-in", "")
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Icon)
                                    .icon("zoom-in")
                                    .on_click(move |_, _, cx| {
                                        cx.update_entity(&state_entity2, |state, _| {
                                            state.zoom_in()
                                        });
                                    }),
                            )
                            .child(
                                div()
                                    .w(px(1.0))
                                    .h(px(20.0))
                                    .bg(gpui::white().opacity(0.2))
                                    .mx(px(8.0)),
                            )
                            .child(
                                Button::new("reset-zoom", "Reset")
                                    .variant(ButtonVariant::Ghost)
                                    .size(ButtonSize::Sm)
                                    .on_click(move |_, _, cx| {
                                        cx.update_entity(&state_entity3, |state, _| {
                                            state.reset_zoom()
                                        });
                                    }),
                            )
                        },
                    ))
                    .child({
                        let viewer_entity = viewer_entity.clone();
                        Button::new("close-viewer", "")
                            .variant(ButtonVariant::Ghost)
                            .size(ButtonSize::Icon)
                            .icon("x")
                            .on_click(move |_, window, cx| {
                                cx.update_entity(&viewer_entity, |viewer, cx| {
                                    viewer.handle_close(window, cx);
                                });
                            })
                    }),
            )
            .child(
                div()
                    .id("image-viewer-content")
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .relative()
                    .overflow_hidden()
                    .when(close_on_backdrop, |this| {
                        let close_handler = close_handler.clone();
                        this.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                            if let Some(handler) = &close_handler {
                                (handler)(window, cx);
                            }
                        })
                    })
                    .when(has_prev, |this| {
                        let state_entity = state_entity.clone();
                        this.child(
                            div()
                                .id("prev-button")
                                .absolute()
                                .left(px(16.0))
                                .top_0()
                                .bottom_0()
                                .flex()
                                .items_center()
                                .child(
                                    Button::new("prev-image", "")
                                        .variant(ButtonVariant::Secondary)
                                        .size(ButtonSize::Icon)
                                        .icon("arrow-left")
                                        .on_click(move |_, _, cx| {
                                            cx.update_entity(&state_entity, |state, _| {
                                                state.prev()
                                            });
                                        }),
                                ),
                        )
                    })
                    .when(has_next, |this| {
                        let state_entity = state_entity.clone();
                        this.child(
                            div()
                                .id("next-button")
                                .absolute()
                                .right(px(16.0))
                                .top_0()
                                .bottom_0()
                                .flex()
                                .items_center()
                                .child(
                                    Button::new("next-image", "")
                                        .variant(ButtonVariant::Secondary)
                                        .size(ButtonSize::Icon)
                                        .icon("arrow-right")
                                        .on_click(move |_, _, cx| {
                                            cx.update_entity(&state_entity, |state, _| {
                                                state.next()
                                            });
                                        }),
                                ),
                        )
                    })
                    .when_some(current_image.clone(), |this, image| {
                        this.child(
                            div()
                                .id("image-container")
                                .flex()
                                .items_center()
                                .justify_center()
                                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                    cx.stop_propagation();
                                })
                                .child(
                                    img(image.src.clone())
                                        .max_w(relative(0.9 * zoom))
                                        .max_h(relative(0.8 * zoom))
                                        .object_fit(ObjectFit::Contain),
                                ),
                        )
                    }),
            )
            .when_some(current_image.as_ref().and_then(|i| i.caption.clone()), {
                |this, caption| {
                    this.child(
                        div()
                            .id("image-caption")
                            .px(px(16.0))
                            .py(px(8.0))
                            .text_size(px(14.0))
                            .text_color(gpui::white().opacity(0.8))
                            .font_family(theme.tokens.font_family.clone())
                            .text_center()
                            .child(caption),
                    )
                }
            })
            .when(show_thumbs, |this| {
                this.child(
                    div()
                        .id("thumbnail-strip")
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap(px(8.0))
                        .px(px(16.0))
                        .py(px(12.0))
                        .bg(gpui::black().opacity(0.5))
                        .children(images.iter().enumerate().map(|(idx, image)| {
                            let is_current = idx == current_index;
                            let state_entity = state_entity.clone();

                            div()
                                .id(ElementId::Name(format!("thumb-{}", idx).into()))
                                .size(px(60.0))
                                .rounded(px(4.0))
                                .overflow_hidden()
                                .border_2()
                                .border_color(if is_current {
                                    theme.tokens.primary
                                } else {
                                    gpui::transparent_black()
                                })
                                .cursor(CursorStyle::PointingHand)
                                .hover(|style| style.opacity(0.8))
                                .on_mouse_down(MouseButton::Left, move |_, _window, cx| {
                                    cx.stop_propagation();
                                    cx.update_entity(&state_entity, |state, _| state.go_to(idx));
                                })
                                .child(
                                    img(image.src.clone())
                                        .size_full()
                                        .object_fit(ObjectFit::Cover),
                                )
                        })),
                )
            })
    }
}

pub fn init_image_viewer(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", ImageViewerClose, Some("ImageViewer")),
        KeyBinding::new("left", ImageViewerPrev, Some("ImageViewer")),
        KeyBinding::new("right", ImageViewerNext, Some("ImageViewer")),
        KeyBinding::new("up", ImageViewerZoomIn, Some("ImageViewer")),
        KeyBinding::new("down", ImageViewerZoomOut, Some("ImageViewer")),
        KeyBinding::new("0", ImageViewerResetZoom, Some("ImageViewer")),
        KeyBinding::new("+", ImageViewerZoomIn, Some("ImageViewer")),
        KeyBinding::new("-", ImageViewerZoomOut, Some("ImageViewer")),
        KeyBinding::new("=", ImageViewerZoomIn, Some("ImageViewer")),
    ]);
}
