use adabraka_ui::{components::scrollable::scrollable_vertical, prelude::*};
use gpui::*;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        std::fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

fn main() {
    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        })
        .run(|cx| {
            adabraka_ui::init(cx);
            adabraka_ui::set_icon_base_path("assets/icons");
            install_theme(cx, Theme::dark());

            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some("HTML Rendering Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| HtmlDemo),
            )
            .unwrap();
        });
}

struct HtmlDemo;

const DEMO_HTML: &str = r#"
<h1>HTML Rendering Demo</h1>
<p>This is a <strong>comprehensive</strong> demonstration of the <code>Html</code> component.</p>

<h2>Inline Formatting</h2>
<p>You can use <strong>bold</strong>, <em>italic</em>, <del>strikethrough</del>, and <code>inline code</code>.</p>

<h2>Links</h2>
<p>Visit <a href="https://example.com">Example</a> or check out <a href="https://www.rust-lang.org">Rust</a>.</p>

<h2>Code Blocks</h2>
<pre><code class="language-rust">fn main() {
    println!("Hello from adabraka-ui!");
    let x = 42;
}</code></pre>

<h2>Lists</h2>
<h3>Unordered</h3>
<ul>
    <li>First item</li>
    <li>Second item with <strong>bold</strong></li>
    <li>Third item with <code>code</code></li>
</ul>

<h3>Ordered</h3>
<ol>
    <li>Step one</li>
    <li>Step two</li>
    <li>Step three</li>
</ol>

<h2>Blockquote</h2>
<blockquote>
    <p>This is a blockquote with <strong>bold</strong> text.</p>
</blockquote>

<h2>Table</h2>
<table>
    <thead>
        <tr>
            <th>Feature</th>
            <th align="center">Status</th>
            <th align="right">Notes</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Bold</td>
            <td align="center">Done</td>
            <td align="right">Works well</td>
        </tr>
        <tr>
            <td>Italic</td>
            <td align="center">Done</td>
            <td align="right">Perfect</td>
        </tr>
    </tbody>
</table>

<h2>Inline CSS Styles</h2>
<p>
    <span style="color: red">Red text</span>,
    <span style="color: #00aa00">Green hex</span>,
    <span style="color: rgb(0, 100, 255)">Blue RGB</span>,
    <span style="color: orange; font-weight: bold">Bold orange</span>,
    <span style="color: purple; font-style: italic">Italic purple</span>.
</p>
<p>
    <span style="background-color: yellow; color: black">Highlighted text</span> and
    <span style="background-color: #333; color: white">Dark background</span>.
</p>
<p>
    <span style="color: coral; font-weight: bold; font-style: italic">Bold italic coral</span> with
    <span style="color: teal">teal</span> and <span style="color: gold; font-weight: bold">gold</span> accents.
</p>

<h2>Images</h2>
<p>Images render natively via GPUI:</p>
<img src="assets/icons/zap.svg" alt="Zap icon">

<hr>
<p>Content below the horizontal rule.</p>
"#;

impl Render for HtmlDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(scrollable_vertical(
                div()
                    .flex()
                    .flex_col()
                    .p(px(32.0))
                    .max_w(px(800.0))
                    .child(Html::new(DEMO_HTML).base_font_size(px(15.0))),
            ))
    }
}
