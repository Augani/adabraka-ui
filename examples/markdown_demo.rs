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
                        title: Some("Markdown Rendering Demo".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(900.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|_| MarkdownDemo),
            )
            .unwrap();
        });
}

struct MarkdownDemo;

const DEMO_MARKDOWN: &str = r#"# Markdown Rendering Demo

This is a **comprehensive** demonstration of the `Markdown` component.

## Inline Formatting

You can use **bold**, *italic*, ~~strikethrough~~, and `inline code` in your text.
You can also combine **bold and *italic*** together.

## Links

Visit [Example](https://example.com) or check out [Rust](https://www.rust-lang.org).

## Code Blocks

```rust
fn main() {
    println!("Hello from adabraka-ui!");
    let x = 42;
}
```

```
Plain code block without language
just some text here
```

## Lists

### Unordered

- First item
- Second item with **bold**
- Third item with `code`

### Ordered

1. Step one
2. Step two
3. Step three

### Task List

- [x] Completed task
- [ ] Pending task
- [x] Another done

## Blockquotes

> This is a blockquote with **bold** text.
> It can span multiple lines.

## Tables

| Feature | Status | Notes |
|---------|:------:|------:|
| Bold | Done | Works well |
| Italic | Done | Perfect |
| Tables | Done | GFM support |
| Links | Done | Clickable |

## Horizontal Rules

Above the rule.

---

Below the rule.

## Headings

### H3 Heading
#### H4 Heading
##### H5 Heading
###### H6 Heading
"#;

impl Render for MarkdownDemo {
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
                    .child(Markdown::new(DEMO_MARKDOWN).base_font_size(px(15.0))),
            ))
    }
}
