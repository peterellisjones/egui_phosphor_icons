//! # egui_phosphor_icons
//!
//! A Rust library providing [Phosphor Icons](https://phosphoricons.com/) for [egui](https://github.com/emilk/egui).
//!
//! ## Features
//!
//! - **Multiple font styles**: Regular, Bold, Fill, Light, and Thin variants
//! - **Feature flags**: Control which font styles are included to reduce binary size
//! - **String lookup**: Look up icons by name for serialization/configuration support
//!
//! ## Quick Start
//!
//! ```no_run
//! use egui_phosphor_icons::{add_fonts, icons, Icon};
//! # fn setup_fonts(ctx: &egui::Context) {
//!
//! // Setup fonts (call once during initialization)
//! let mut fonts = egui::FontDefinitions::default();
//! add_fonts(&mut fonts);
//! ctx.set_fonts(fonts);
//! # }
//! # fn ui_example(ui: &mut egui::Ui) {
//!
//! // Use icons in your UI
//! ui.label(icons::HOUSE);              // Regular style (default)
//! ui.label(icons::HOUSE.bold());       // Bold style
//! ui.label(icons::HOUSE.fill());       // Fill style
//! ui.label(icons::HOUSE.light());      // Light style
//! ui.label(icons::HOUSE.thin());       // Thin style
//!
//! // Chain with RichText methods
//! ui.label(icons::HEART.fill().color(egui::Color32::RED).size(32.0));
//!
//! // Look up icons by string name (kebab-case)
//! if let Some(icon) = Icon::from_name("arrow-up-left") {
//!     ui.label(icon.regular());
//! }
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! By default, all font styles are included. You can disable specific styles to reduce binary size:
//!
//! ```toml
//! # Only include regular and bold styles
//! egui_phosphor_icons = { version = "0.1.0", default-features = false, features = ["bold"] }
//!
//! # Only regular style (smallest binary size)
//! egui_phosphor_icons = { version = "0.1.0", default-features = false }
//! ```
//!
//! Available features:
//! - `bold` - Bold font variant
//! - `fill` - Fill font variant
//! - `light` - Light font variant
//! - `thin` - Thin font variant
//!
//! ## Usage with bevy_egui
//!
//! When using this library with [bevy_egui](https://github.com/mvlabat/bevy_egui), configure fonts during initialization:
//!
//! ```ignore
//! use bevy::prelude::*;
//! use bevy_egui::{EguiContext, EguiPlugin, PrimaryEguiContext, egui};
//! use egui_phosphor_icons::{add_fonts, icons};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(EguiPlugin)
//!         .add_systems(Update, configure_fonts)
//!         .run();
//! }
//!
//! // Configure fonts once when EguiContext is first added
//! fn configure_fonts(mut egui_contexts: Query<&mut EguiContext, Added<PrimaryEguiContext>>) {
//!     let Some(mut ctx) = egui_contexts.iter_mut().next() else {
//!         return;
//!     };
//!
//!     let mut fonts = egui::FontDefinitions::default();
//!     add_fonts(&mut fonts);
//!     ctx.get_mut().set_fonts(fonts);
//! }
//! ```

pub mod icons;

use std::sync::Arc;

// Font family name constants - used internally
const PHOSPHOR_REGULAR_NAME: &str = "phosphor-regular";

#[cfg(feature = "bold")]
const PHOSPHOR_BOLD_NAME: &str = "phosphor-bold";

#[cfg(feature = "fill")]
const PHOSPHOR_FILL_NAME: &str = "phosphor-fill";

#[cfg(feature = "light")]
const PHOSPHOR_LIGHT_NAME: &str = "phosphor-light";

#[cfg(feature = "thin")]
const PHOSPHOR_THIN_NAME: &str = "phosphor-thin";

/// Adds Phosphor Icons font to egui font definitions.
///
/// This function loads the Phosphor Icons font and creates separate font families
/// for each variant, allowing you to choose which style to use when rendering icons.
///
/// # Example
/// ```no_run
/// use egui_phosphor_icons::{add_fonts, icons};
/// # fn setup_fonts(ctx: &egui::Context) {
///
/// let mut fonts = egui::FontDefinitions::default();
/// add_fonts(&mut fonts);
/// // Then set the fonts on your context:
/// ctx.set_fonts(fonts);
/// # }
/// # fn ui_example(ui: &mut egui::Ui) {
///
/// // Use icons with different styles:
/// ui.label(icons::HOUSE.regular());
/// ui.label(icons::HOUSE.bold());  // Requires "bold" feature
/// ui.label(icons::HOUSE.fill());  // Requires "fill" feature
/// # }
/// ```
pub fn add_fonts(fonts: &mut egui::FontDefinitions) {
    // Load phosphor icons fonts (regular is always included)
    let phosphor_font_data = include_bytes!("../assets/fonts/Phosphor.ttf");
    fonts.font_data.insert(
        "phosphor-icons".to_owned(),
        Arc::new(egui::FontData::from_static(phosphor_font_data)),
    );

    #[cfg(feature = "bold")]
    {
        let phosphor_bold_font_data = include_bytes!("../assets/fonts/Phosphor-Bold.ttf");
        fonts.font_data.insert(
            "phosphor-icons-bold".to_owned(),
            Arc::new(egui::FontData::from_static(phosphor_bold_font_data)),
        );
    }

    #[cfg(feature = "fill")]
    {
        let phosphor_fill_font_data = include_bytes!("../assets/fonts/Phosphor-Fill.ttf");
        fonts.font_data.insert(
            "phosphor-icons-fill".to_owned(),
            Arc::new(egui::FontData::from_static(phosphor_fill_font_data)),
        );
    }

    #[cfg(feature = "light")]
    {
        let phosphor_light_font_data = include_bytes!("../assets/fonts/Phosphor-Light.ttf");
        fonts.font_data.insert(
            "phosphor-icons-light".to_owned(),
            Arc::new(egui::FontData::from_static(phosphor_light_font_data)),
        );
    }

    #[cfg(feature = "thin")]
    {
        let phosphor_thin_font_data = include_bytes!("../assets/fonts/Phosphor-Thin.ttf");
        fonts.font_data.insert(
            "phosphor-icons-thin".to_owned(),
            Arc::new(egui::FontData::from_static(phosphor_thin_font_data)),
        );
    }

    // Create separate font families for each variant
    // Regular font family
    fonts.families.insert(
        egui::FontFamily::Name(PHOSPHOR_REGULAR_NAME.into()),
        vec!["phosphor-icons".to_owned()],
    );

    #[cfg(feature = "bold")]
    fonts.families.insert(
        egui::FontFamily::Name(PHOSPHOR_BOLD_NAME.into()),
        vec!["phosphor-icons-bold".to_owned()],
    );

    #[cfg(feature = "fill")]
    fonts.families.insert(
        egui::FontFamily::Name(PHOSPHOR_FILL_NAME.into()),
        vec!["phosphor-icons-fill".to_owned()],
    );

    #[cfg(feature = "light")]
    fonts.families.insert(
        egui::FontFamily::Name(PHOSPHOR_LIGHT_NAME.into()),
        vec!["phosphor-icons-light".to_owned()],
    );

    #[cfg(feature = "thin")]
    fonts.families.insert(
        egui::FontFamily::Name(PHOSPHOR_THIN_NAME.into()),
        vec!["phosphor-icons-thin".to_owned()],
    );
}

/// A Phosphor icon that can be rendered with different font styles.
///
/// # Example
/// ```no_run
/// use egui_phosphor_icons::icons;
///
/// // In your egui UI code:
/// // ui.label(icons::HOUSE.regular());
/// // ui.label(icons::HOUSE.bold());
/// // ui.label(icons::HOUSE.fill());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Icon(pub &'static str);

impl Icon {
    /// Creates a new Icon from a unicode string.
    pub const fn new(icon: &'static str) -> Self {
        Self(icon)
    }

    /// Returns the raw unicode string for this icon.
    ///
    /// Useful for string concatenation when you want to combine icons with text.
    /// Note that the combined string will use whatever font is applied to it,
    /// so you'll need to use a font family that includes the Phosphor icons.
    ///
    /// # Example
    /// ```no_run
    /// use egui_phosphor_icons::icons;
    ///
    /// // Combine icon with text
    /// let text = format!("{} Settings", icons::GEAR.as_str());
    /// // ui.label(text);
    /// ```
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Renders the icon using the regular font style.
    pub fn regular(self) -> egui::RichText {
        egui::RichText::new(self.0).family(egui::FontFamily::Name(PHOSPHOR_REGULAR_NAME.into()))
    }

    /// Renders the icon using the bold font style.
    ///
    /// Only available with the `bold` feature enabled.
    #[cfg(feature = "bold")]
    pub fn bold(self) -> egui::RichText {
        egui::RichText::new(self.0).family(egui::FontFamily::Name(PHOSPHOR_BOLD_NAME.into()))
    }

    /// Renders the icon using the fill font style.
    ///
    /// Only available with the `fill` feature enabled.
    #[cfg(feature = "fill")]
    pub fn fill(self) -> egui::RichText {
        egui::RichText::new(self.0).family(egui::FontFamily::Name(PHOSPHOR_FILL_NAME.into()))
    }

    /// Renders the icon using the light font style.
    ///
    /// Only available with the `light` feature enabled.
    #[cfg(feature = "light")]
    pub fn light(self) -> egui::RichText {
        egui::RichText::new(self.0).family(egui::FontFamily::Name(PHOSPHOR_LIGHT_NAME.into()))
    }

    /// Renders the icon using the thin font style.
    ///
    /// Only available with the `thin` feature enabled.
    #[cfg(feature = "thin")]
    pub fn thin(self) -> egui::RichText {
        egui::RichText::new(self.0).family(egui::FontFamily::Name(PHOSPHOR_THIN_NAME.into()))
    }

    /// Looks up an icon by its string name using kebab-case (e.g., "arrow-up-left").
    ///
    /// Returns `Some(Icon)` if the name exists, `None` otherwise.
    /// This is useful for loading icons from serializable formats like JSON or configuration files.
    ///
    /// # Example
    /// ```
    /// use egui_phosphor_icons::Icon;
    ///
    /// let icon = Icon::from_name("arrow-up-left").unwrap();
    /// // ui.label(icon.regular());
    ///
    /// let icon = Icon::from_name("house").unwrap();
    /// // ui.label(icon.fill());
    /// ```
    pub fn from_name(name: &str) -> Option<Icon> {
        icons::icon_map().get(name).copied()
    }

    /// Returns an iterator over all available icon names.
    ///
    /// Useful for debugging, documentation, or building icon pickers.
    ///
    /// # Example
    /// ```
    /// use egui_phosphor_icons::Icon;
    ///
    /// for name in Icon::names() {
    ///     println!("{}", name);
    /// }
    /// ```
    pub fn names() -> impl Iterator<Item = &'static str> {
        icons::icon_map().keys().copied()
    }
}

/// Converts an Icon to RichText using the regular font style by default.
///
/// This allows Icons to be used directly where RichText is expected.
///
/// # Example
/// ```no_run
/// use egui_phosphor_icons::icons;
///
/// let rich_text: egui::RichText = icons::HOUSE.into();
/// ```
impl From<Icon> for egui::RichText {
    fn from(icon: Icon) -> Self {
        icon.regular()
    }
}

/// Converts an Icon to WidgetText using the regular font style by default.
///
/// This allows Icons to be used directly in any egui widget that accepts WidgetText.
///
/// # Example
/// ```no_run
/// use egui_phosphor_icons::icons;
/// # fn ui_example(ui: &mut egui::Ui) {
///
/// // Icons work directly in labels, buttons, etc.
/// ui.label(icons::HOUSE);
/// ui.button(icons::GEAR);
/// # }
/// ```
impl From<Icon> for egui::WidgetText {
    fn from(icon: Icon) -> Self {
        icon.regular().into()
    }
}
