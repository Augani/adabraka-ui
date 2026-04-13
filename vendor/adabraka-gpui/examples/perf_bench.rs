use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

const ROW_COUNT: usize = 100;
const COL_COUNT: usize = 30;

static STARTED: AtomicBool = AtomicBool::new(false);

struct PerfBench {
    frame_count: u64,
    total_frames: u64,
    last_report: Instant,
    start_time: Instant,
    fps_text: SharedString,
    log_file: std::fs::File,
}

impl Render for PerfBench {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !STARTED.swap(true, Ordering::Relaxed) {
            let _ = writeln!(self.log_file, "render started");
            let _ = self.log_file.flush();
        }
        self.frame_count += 1;
        self.total_frames += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_report);
        if elapsed.as_secs_f64() >= 1.0 {
            let fps = self.frame_count as f64 / elapsed.as_secs_f64();
            let total_elapsed = now.duration_since(self.start_time).as_secs_f64();
            let avg_fps = self.total_frames as f64 / total_elapsed;
            let _ = writeln!(
                self.log_file,
                "FPS: {:.1} | Avg: {:.1} | Elements: {} | Frame: {}",
                fps,
                avg_fps,
                ROW_COUNT * COL_COUNT,
                self.total_frames
            );
            let _ = self.log_file.flush();
            self.fps_text = format!("FPS: {:.1} | Elements: {}", fps, ROW_COUNT * COL_COUNT).into();
            self.frame_count = 0;
            self.last_report = now;
        }

        window.request_animation_frame();

        let colors = [
            rgb(0xE74C3C),
            rgb(0x3498DB),
            rgb(0x2ECC71),
            rgb(0xF39C12),
            rgb(0x9B59B6),
            rgb(0x1ABC9C),
        ];

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1a1a2e))
            .child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .bg(rgb(0x16213e))
                    .text_color(rgb(0x00ff88))
                    .text_sm()
                    .child(self.fps_text.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .p(px(8.0))
                    .flex_grow()
                    .children((0..ROW_COUNT).map(|row| {
                        div()
                            .flex()
                            .gap(px(2.0))
                            .children((0..COL_COUNT).map(|col| {
                                let color = colors[(row + col) % colors.len()];
                                let cell_text: SharedString =
                                    format!("{}", (row * COL_COUNT + col) % 100).into();

                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .size(px(32.0))
                                    .bg(color)
                                    .rounded(px(4.0))
                                    .border_1()
                                    .border_color(gpui::hsla(0.0, 0.0, 1.0, 0.2))
                                    .shadow_sm()
                                    .text_color(rgb(0xffffff))
                                    .text_xs()
                                    .child(cell_text)
                            }))
                    })),
            )
    }
}

fn main() {
    println!("perf_bench starting...");
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let log_path =
                    std::env::var("PERF_LOG").unwrap_or_else(|_| "/tmp/gpui_perf.log".to_string());
                let log_file = std::fs::File::create(&log_path).expect("failed to create log file");
                cx.new(|_| PerfBench {
                    frame_count: 0,
                    total_frames: 0,
                    last_report: Instant::now(),
                    start_time: Instant::now(),
                    fps_text: "Measuring...".into(),
                    log_file,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
