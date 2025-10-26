# adabraka-ui v0.1.0 🎉

We're excited to announce the first public release of **adabraka-ui** - a comprehensive, professional UI component library for [GPUI](https://github.com/zed-industries/zed)!

## 🌟 What is adabraka-ui?

adabraka-ui is a complete UI toolkit for building beautiful desktop applications with Rust and GPUI. Inspired by [shadcn/ui](https://ui.shadcn.com/), it provides 70+ polished, accessible components with a modern design system.

## ✨ Highlights

### 🎨 Complete UI System
- **70+ Components** covering all your UI needs
- **Built-in Theme System** with light/dark modes and semantic color tokens
- **Professional Animations** with cubic-bezier easing and spring physics
- **Typography System** with semantic text variants
- **Lucide Icons** integration

### 📱 Real-World Examples
Check out these apps built with adabraka-ui:
- **Desktop Music Player** - Beautiful offline music player with smooth animations
- **Task Manager** - Used to track the development of this library!
- **50+ Component Demos** - Learn from comprehensive examples

### 🧩 Component Categories

**Input & Forms**
- Button, IconButton, Input, Textarea
- Checkbox, Toggle, Radio, Select
- SearchInput, Editor (with syntax highlighting)

**Navigation**
- Sidebar, MenuBar, Tabs, Breadcrumbs
- Tree, Toolbar, StatusBar, Command Palette

**Data Display**
- Table, DataTable (virtual scrolling)
- Card, Badge, Accordion, Progress

**Overlays**
- Dialog, Popover, Tooltip, Toast
- Alert, Context Menu

**Layout**
- VStack, HStack, Grid
- Scrollable, Resizable Panels

### 🎯 Key Features

- ✅ **Type-Safe** - Leverages Rust's type system for compile-time guarantees
- ✅ **Accessible** - Full keyboard navigation, ARIA labels, screen reader support
- ✅ **High Performance** - Optimized for GPUI's retained-mode rendering
- ✅ **Well Documented** - Comprehensive examples and API documentation
- ✅ **Builder Pattern** - Ergonomic, chainable API design

## 📦 Installation

> **Note:** Currently requires Rust nightly due to GPUI dependencies.

```toml
[dependencies]
adabraka-ui = "0.1.0"
gpui = "0.2.0"
```

```bash
cargo +nightly build
```

## 🚀 Quick Start

```rust
use adabraka_ui::prelude::*;
use gpui::*;

fn main() {
    Application::new().run(|cx| {
        adabraka_ui::init(cx);
        install_theme(cx, Theme::dark());

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("My App".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MyApp::new()),
        ).unwrap();
    });
}
```

## 🌐 Resources

- **Website**: https://augani.github.io/adabraka-ui
- **Documentation**: https://docs.rs/adabraka-ui
- **Crates.io**: https://crates.io/crates/adabraka-ui
- **Examples**: Browse 50+ examples in the `examples/` directory

## 📚 Documentation

Visit our [beautiful documentation site](https://augani.github.io/adabraka-ui) featuring:
- Real app showcases
- Installation guide
- Component examples with code
- Comprehensive API reference

## 🤝 Contributing

We welcome contributions! Check out our [Contributing Guide](CONTRIBUTING.md) to get started.

- **Bug Reports**: Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- **Feature Requests**: Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- **Pull Requests**: Follow our [PR template](.github/PULL_REQUEST_TEMPLATE.md)

## 🙏 Acknowledgments

Special thanks to:
- **[Zed Industries](https://zed.dev/)** - For creating GPUI
- **[Lucide Icons](https://lucide.dev/)** - For the beautiful icon set
- **[shadcn/ui](https://ui.shadcn.com/)** - For design inspiration

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ using GPUI and inspired by shadcn/ui**
