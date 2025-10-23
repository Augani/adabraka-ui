use adabraka_ui::{
    prelude::*,
    components::scrollable::scrollable_vertical,
};
use gpui::{prelude::FluentBuilder, *};
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
                        title: Some("Combobox Demo - Adabraka UI".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
                        size: size(px(1000.0), px(900.0)),
                    })),
                    ..Default::default()
                },
                |_, cx| cx.new(|cx| ComboboxDemo::new(cx)),
            )
            .unwrap();
        });
}

// Custom struct for demonstrating structured data
#[derive(Clone, Debug, PartialEq)]
struct Person {
    id: usize,
    name: String,
    age: u32,
    role: String,
}

impl Person {
    fn new(id: usize, name: &str, age: u32, role: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
            role: role.to_string(),
        }
    }
}

// Framework options
#[derive(Clone, Debug, PartialEq)]
struct Framework {
    name: String,
    category: String,
}

impl Framework {
    fn new(name: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

struct ComboboxDemo {
    // Combobox components
    fruits_combobox: Entity<Combobox<String>>,
    people_combobox: Entity<Combobox<Person>>,
    frameworks_combobox: Entity<Combobox<Framework>>,
    colors_combobox: Entity<Combobox<String>>,
    disabled_combobox: Entity<Combobox<String>>,

    // Selected value display
    selected_fruit: Option<String>,
    selected_person: Option<String>,
    selected_framework: Option<String>,
    selected_colors: String,
}

impl ComboboxDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let fruits = vec![
            "Apple".to_string(),
            "Banana".to_string(),
            "Cherry".to_string(),
            "Date".to_string(),
            "Elderberry".to_string(),
            "Fig".to_string(),
            "Grape".to_string(),
            "Honeydew".to_string(),
            "Kiwi".to_string(),
            "Lemon".to_string(),
            "Mango".to_string(),
            "Nectarine".to_string(),
            "Orange".to_string(),
            "Papaya".to_string(),
            "Quince".to_string(),
        ];

        let people = vec![
            Person::new(1, "Alice Johnson", 28, "Developer"),
            Person::new(2, "Bob Smith", 35, "Designer"),
            Person::new(3, "Charlie Brown", 42, "Manager"),
            Person::new(4, "Diana Prince", 30, "Developer"),
            Person::new(5, "Eve Adams", 26, "Designer"),
            Person::new(6, "Frank Miller", 38, "Developer"),
            Person::new(7, "Grace Lee", 33, "Manager"),
            Person::new(8, "Henry Wilson", 29, "Developer"),
        ];

        let frameworks = vec![
            Framework::new("React", "Frontend"),
            Framework::new("Vue", "Frontend"),
            Framework::new("Angular", "Frontend"),
            Framework::new("Svelte", "Frontend"),
            Framework::new("Node.js", "Backend"),
            Framework::new("Django", "Backend"),
            Framework::new("Rails", "Backend"),
            Framework::new("Laravel", "Backend"),
            Framework::new("Spring", "Backend"),
        ];

        let colors = vec![
            "Red".to_string(),
            "Blue".to_string(),
            "Green".to_string(),
            "Yellow".to_string(),
            "Purple".to_string(),
            "Orange".to_string(),
            "Pink".to_string(),
            "Brown".to_string(),
            "Black".to_string(),
            "White".to_string(),
        ];

        let disabled_items = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
        ];

        // Create state entities
        let fruits_state = cx.new(|_| ComboboxState::new());
        let people_state = cx.new(|_| ComboboxState::new());
        let frameworks_state = cx.new(|_| ComboboxState::new());
        let colors_state = cx.new(|_| ComboboxState::new());
        let disabled_state = cx.new(|_| ComboboxState::new());

        // Get entity reference for callbacks
        let demo_entity = cx.entity().clone();

        // Create Combobox entities
        let fruits_combobox = cx.new(|cx| {
            let entity = demo_entity.clone();
            Combobox::new(fruits.clone(), &fruits_state, cx)
                .placeholder("Select a fruit...")
                .filter_fn(|item, search| {
                    item.to_lowercase().contains(&search.to_lowercase())
                })
                .render_item(|item| item.clone().into())
                .on_select(move |item, _, cx| {
                    entity.update(cx, |demo, cx| {
                        demo.selected_fruit = Some(item.clone());
                        cx.notify();
                    })
                })
                .w_full()
        });

        let people_combobox = cx.new(|cx| {
            let entity = demo_entity.clone();
            Combobox::new(people.clone(), &people_state, cx)
                .placeholder("Search people...")
                .filter_fn(|person, search| {
                    let s = search.to_lowercase();
                    person.name.to_lowercase().contains(&s)
                        || person.role.to_lowercase().contains(&s)
                })
                .render_item(|person| {
                    format!("{} - {} ({})", person.name, person.role, person.age).into()
                })
                .render_selected(|people| {
                    if people.is_empty() {
                        "Select a person...".into()
                    } else {
                        people[0].name.clone().into()
                    }
                })
                .on_select(move |person, _, cx| {
                    entity.update(cx, |demo, cx| {
                        demo.selected_person = Some(format!(
                            "{} - {} years old, working as {}",
                            person.name, person.age, person.role
                        ));
                        cx.notify();
                    })
                })
                .w_full()
        });

        let frameworks_combobox = cx.new(|cx| {
            let entity = demo_entity.clone();
            Combobox::new(frameworks.clone(), &frameworks_state, cx)
                .placeholder("Select framework...")
                .filter_fn(|fw, search| {
                    let s = search.to_lowercase();
                    fw.name.to_lowercase().contains(&s)
                        || fw.category.to_lowercase().contains(&s)
                })
                .render_item(|fw| {
                    format!("{} ({})", fw.name, fw.category).into()
                })
                .on_select(move |fw, _, cx| {
                    entity.update(cx, |demo, cx| {
                        demo.selected_framework = Some(format!(
                            "{} - {} Framework",
                            fw.name, fw.category
                        ));
                        cx.notify();
                    })
                })
                .w_full()
                .bg(rgb(0x1e3a8a))
                .text_color(gpui::white())
                .rounded(px(12.0))
                .border_2()
                .border_color(rgb(0x3b82f6))
        });

        let colors_combobox = cx.new(|cx| {
            let entity = demo_entity.clone();
            let colors_state_clone = colors_state.clone();
            Combobox::new(colors.clone(), &colors_state, cx)
                .placeholder("Select colors...")
                .multi_select(true)
                .filter_fn(|color, search| {
                    color.to_lowercase().contains(&search.to_lowercase())
                })
                .render_item(|color| color.clone().into())
                .render_selected(|colors| {
                    if colors.is_empty() {
                        "Select colors...".into()
                    } else {
                        format!("{} color(s) selected", colors.len()).into()
                    }
                })
                .on_select(move |_, _, cx| {
                    entity.update(cx, |demo, cx| {
                        let state = colors_state_clone.read(cx);
                        if state.selected.is_empty() {
                            demo.selected_colors = "None selected".to_string();
                        } else {
                            demo.selected_colors = state.selected.join(", ");
                        }
                        cx.notify();
                    })
                })
                .w_full()
        });

        let disabled_combobox = cx.new(|cx| {
            Combobox::new(disabled_items.clone(), &disabled_state, cx)
                .placeholder("This is disabled...")
                .disabled(true)
                .filter_fn(|item, search| {
                    item.to_lowercase().contains(&search.to_lowercase())
                })
                .render_item(|item| item.clone().into())
                .w_full()
        });

        Self {
            fruits_combobox,
            people_combobox,
            frameworks_combobox,
            colors_combobox,
            disabled_combobox,

            selected_fruit: None,
            selected_person: None,
            selected_framework: None,
            selected_colors: "None selected".to_string(),
        }
    }
}

impl Render for ComboboxDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = use_theme();

        div()
            .size_full()
            .bg(theme.tokens.background)
            .overflow_hidden()
            .child(
                scrollable_vertical(
                    div()
                        .flex()
                        .flex_col()
                        .text_color(theme.tokens.foreground)
                        .p(px(32.0))
                        .gap(px(32.0))
                        // Header
                        .child(
                            VStack::new()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(28.0))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Combobox Component Showcase")
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Searchable dropdown with keyboard navigation, filtering, and customization")
                                )
                        )
                        // 1. Basic String Combobox
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("1. Basic String Combobox")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Simple searchable list of fruits. Type to filter results.")
                                )
                                .child(self.fruits_combobox.clone())
                                .when(self.selected_fruit.is_some(), |stack| {
                                    stack.child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.accent)
                                            .rounded(px(6.0))
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child(format!("Selected: {}", self.selected_fruit.as_ref().unwrap()))
                                    )
                                })
                        )
                        // 2. Custom Struct Combobox (Person)
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("2. Custom Struct Combobox")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Combobox with Person objects. Search by name or role.")
                                )
                                .child(self.people_combobox.clone())
                                .when(self.selected_person.is_some(), |stack| {
                                    stack.child(
                                        div()
                                            .p(px(12.0))
                                            .bg(theme.tokens.accent)
                                            .rounded(px(6.0))
                                            .text_size(px(13.0))
                                            .text_color(theme.tokens.accent_foreground)
                                            .child(format!("Selected: {}", self.selected_person.as_ref().unwrap()))
                                    )
                                })
                        )
                        // 3. Styled Combobox (Frameworks)
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("3. Styled Combobox with Custom Colors")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Combobox with custom styling using Styled trait. Search frameworks by name or category.")
                                )
                                .child(self.frameworks_combobox.clone())
                                .when(self.selected_framework.is_some(), |stack| {
                                    stack.child(
                                        div()
                                            .p(px(12.0))
                                            .bg(rgb(0x1e3a8a))
                                            .border_1()
                                            .border_color(rgb(0x3b82f6))
                                            .rounded(px(6.0))
                                            .text_size(px(13.0))
                                            .text_color(gpui::white())
                                            .child(format!("Selected: {}", self.selected_framework.as_ref().unwrap()))
                                    )
                                })
                        )
                        // 4. Multi-Select Combobox
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("4. Multi-Select Combobox")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Select multiple colors. Use Cmd/Ctrl+K to clear all.")
                                )
                                .child(self.colors_combobox.clone())
                                .child(
                                    div()
                                        .p(px(12.0))
                                        .bg(theme.tokens.muted)
                                        .rounded(px(6.0))
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child(format!("Selected colors: {}", self.selected_colors))
                                )
                        )
                        // 5. Disabled Combobox
                        .child(
                            VStack::new()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("5. Disabled Combobox")
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(theme.tokens.muted_foreground)
                                        .child("Combobox in disabled state - not interactive.")
                                )
                                .child(self.disabled_combobox.clone())
                        )
                        // Info Box
                        .child(
                            div()
                                .mt(px(16.0))
                                .p(px(20.0))
                                .bg(theme.tokens.primary.opacity(0.1))
                                .border_1()
                                .border_color(theme.tokens.primary)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.primary)
                                                .child("Keyboard Shortcuts")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("• Up/Down arrows - Navigate options")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("• Enter - Select highlighted option")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("• Escape - Close dropdown")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("• Type - Filter options in real-time")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.foreground)
                                                .child("• Cmd/Ctrl+K - Clear selection (multi-select)")
                                        )
                                )
                        )
                        // Features List
                        .child(
                            div()
                                .p(px(20.0))
                                .bg(theme.tokens.accent)
                                .rounded(px(8.0))
                                .child(
                                    VStack::new()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("Component Features")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Real-time search filtering")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Full keyboard navigation")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Custom filter and render functions")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Multi-selection mode")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Clear button for easy reset")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Full Styled trait support")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Virtual scrolling for large lists")
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(theme.tokens.accent_foreground)
                                                .child("✓ Disabled state support")
                                        )
                                )
                        )
                )
            )
    }
}
