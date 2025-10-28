# egui_phosphor_icons

A Rust library providing [Phosphor Icons](https://phosphoricons.com/) for [egui](https://github.com/emilk/egui).

## Features

- **Multiple font styles**: Regular, Bold, Fill, Light, and Thin variants
- **Feature flags**: Control which font styles are included to reduce binary size

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui_phosphor_icons = "*"
```

## Quick Start

```rust
use egui_phosphor_icons::{add_fonts, icons};

// Setup fonts (call once during initialization)
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    add_fonts(&mut fonts);
    ctx.set_fonts(fonts);
}

// Use icons in your UI
fn ui_example(ui: &mut egui::Ui) {
    // Simple usage - regular style by default
    ui.label(icons::HOUSE);

    // Different styles
    ui.label(icons::HOUSE.regular());
    ui.label(icons::HOUSE.bold());
    ui.label(icons::HOUSE.fill());
    ui.label(icons::HOUSE.light());
    ui.label(icons::HOUSE.thin());

    // Chain with RichText methods
    ui.label(icons::HEART.fill().color(egui::Color32::RED).size(32.0));

    // Use in buttons
    if ui.button(icons::GEAR.regular()).clicked() {
        // Settings clicked
    }

    // Combine with text
    ui.horizontal(|ui| {
        ui.label(icons::MAGNIFYING_GLASS);
        ui.label("Search");
    });
}
```

## Feature Flags

By default, all font styles are included. You can disable specific styles to reduce binary size:

```toml
# Only include regular and bold styles
egui_phosphor_icons = { version = "*", default-features = false, features = ["bold"] }

# Only regular style (smallest binary size)
egui_phosphor_icons = { version = "*", default-features = false }

# Custom selection
egui_phosphor_icons = { version = "*", default-features = false, features = ["bold", "fill"] }
```

Available features:

- `bold` - Bold font variant
- `fill` - Fill font variant
- `light` - Light font variant
- `thin` - Thin font variant

## Available Icons

This library includes all icons from [Phosphor Icons](https://phosphoricons.com/). Some examples:

- **UI Elements**: `HOUSE`, `GEAR`, `USER`, `BELL`, `HEART`, `STAR`
- **Actions**: `PLAY`, `PAUSE`, `STOP`, `TRASH`, `PENCIL`, `CHECK`
- **Navigation**: `ARROW_LEFT`, `ARROW_RIGHT`, `CARET_DOWN`, `X`
- **Communication**: `CHAT`, `PHONE`, `ENVELOPE`, `PAPER_PLANE`
- **Media**: `IMAGE`, `VIDEO_CAMERA`, `MUSIC_NOTE`, `FILE`
- **And many more...**

See the full list of available icons at [phosphoricons.com](https://phosphoricons.com/) or in the `icons` module documentation.

## API Reference

### `add_fonts(fonts: &mut egui::FontDefinitions)`

Adds Phosphor Icons fonts to your egui font definitions. Call this once during initialization before setting fonts on your context.

### `Icon`

A type representing a Phosphor icon. All icons in the `icons` module are of this type.

#### Methods

- `.regular()` -> `RichText` - Renders with regular style
- `.bold()` -> `RichText` - Renders with bold style (requires `bold` feature)
- `.fill()` -> `RichText` - Renders with fill style (requires `fill` feature)
- `.light()` -> `RichText` - Renders with light style (requires `light` feature)
- `.thin()` -> `RichText` - Renders with thin style (requires `thin` feature)

Icons automatically convert to `RichText` with regular style via `Into`, so you can use them directly:

```rust
ui.label(icons::HOUSE);  // Equivalent to icons::HOUSE.regular()
```

## Usage with bevy_egui

When using this library with [bevy_egui](https://github.com/mvlabat/bevy_egui), you need to configure the fonts during initialization. Here's a complete example:

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, PrimaryEguiContext, egui};
use egui_phosphor_icons::{add_fonts, icons};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, configure_fonts)
        .add_systems(Update, ui_system)
        .run();
}

// Configure fonts once when the EguiContext is first added
fn configure_fonts(mut egui_contexts: Query<&mut EguiContext, Added<PrimaryEguiContext>>) {
    let Some(mut ctx) = egui_contexts.iter_mut().next() else {
        return;
    };

    let mut fonts = egui::FontDefinitions::default();
    add_fonts(&mut fonts);
    ctx.get_mut().set_fonts(fonts);
}

// Now you can use icons in any UI system
fn ui_system(mut egui_contexts: Query<&mut EguiContext>) {
    let Some(mut ctx) = egui_contexts.iter_mut().next() else {
        return;
    };

    egui::Window::new("My Window").show(ctx.get_mut(), |ui| {
        if ui.button(icons::GEAR.regular()).clicked() {
            // Settings button clicked
        }

        ui.label(icons::HEART.fill().color(egui::Color32::RED));
    });
}
```

**Key points for bevy_egui integration:**
- Add the font configuration in a system that runs on `Added<PrimaryEguiContext>` to ensure it only runs once
- Query for `&mut EguiContext` and call `ctx.get_mut()` to access the egui context
- Icons can then be used normally in any system that has access to the egui context

## Examples

### Basic Icon Display

```rust
ui.label(icons::SMILEY);
```

### Styled Icons

```rust
ui.label(icons::WARNING.fill().color(egui::Color32::YELLOW).size(24.0));
ui.label(icons::CHECK_CIRCLE.fill().color(egui::Color32::GREEN));
ui.label(icons::X_CIRCLE.fill().color(egui::Color32::RED));
```

### Icon Buttons

```rust
if ui.button(icons::PLUS.bold()).clicked() {
    // Add item
}

if ui.button(icons::TRASH.regular()).clicked() {
    // Delete item
}
```

### Icon with Text

```rust
ui.horizontal(|ui| {
    ui.label(icons::INFO);
    ui.label("This is an information message");
});
```

### Different Styles

```rust
ui.horizontal(|ui| {
    ui.label(icons::HEART.regular());
    ui.label(icons::HEART.bold());
    ui.label(icons::HEART.fill());
    ui.label(icons::HEART.light());
    ui.label(icons::HEART.thin());
});
```

## License

This library packages the Phosphor Icons font files. Phosphor Icons are licensed under the MIT License. See the [Phosphor Icons repository](https://github.com/phosphor-icons/core) for details.
