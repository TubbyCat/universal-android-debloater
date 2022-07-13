use crate::core::theme::Theme;
use iced::{text, application, button, checkbox, container, pick_list, scrollable, text_input};
use iced::{Background, Color};
use iced::overlay::menu;

#[derive(Debug, Clone, Copy)]
pub enum Application {
    Default,
    Custom(fn(Theme) -> application::Appearance),
}

impl Default for Application {
    fn default() -> Self {
        Self::Default
    }
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, style: Self::Style) -> application::Appearance {
        let palette = self.palette;

        match style {
            Application::Default => application::Appearance {
                background_color: palette.base.background,
                text_color: palette.base.foreground,
            },
            Application::Custom(f) => f(*self),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Container {
    Content,
    Navigation,
    Description
}

impl Default for Container {
    fn default() -> Self {
        Self::Content
    }
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: Self::Style) -> container::Appearance {
        match style {
            Content => container::Appearance {
                background: Some(Background::Color(self.palette.base.background)),
                text_color: Some(self.palette.bright.surface),
                ..container::Appearance::default()
            },
            Navigation => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.bright.surface),
                ..container::Appearance::default()
            },
            Description => container::Appearance {
                background: Some(Background::Color(self.palette.base.foreground)),
                text_color: Some(self.palette.bright.surface),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: self.palette.base.background,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Primary,
    Unavailable,
    SelfUpdate,
    Refresh,
    UninstallPackage,
    RestorePackage,
    NormalPackage,
    SelectedPackage
}

impl Default for Button {
    fn default() -> Self {
        Self::Primary
    }
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: Self::Style) -> button::Appearance {
        let palette = self.palette;

        let appearance = button::Appearance {
            border_color: Color {
                a: 0.5,
                ..palette.bright.primary
            },
            border_width: 1.0,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        let from_pair = |pair: iced::Color| button::Appearance {
            background: Some(pair.into()),
            text_color: pair,
            ..appearance
        };

        match style {
            Button::Primary => from_pair(palette.bright.primary),
            Button::Unavailable => from_pair(palette.bright.error),
            Button::Refresh => from_pair(palette.bright.secondary),
            Button::SelfUpdate => from_pair(palette.bright.primary),
            Button::UninstallPackage => from_pair(palette.bright.secondary),
            Button::RestorePackage => from_pair(palette.bright.secondary),
            Button::RestorePackage => from_pair(palette.bright.secondary),
            Button::NormalPackage => button::Appearance {
                background: Some(Background::Color(palette.base.foreground)),
                text_color: palette.bright.surface,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: palette.base.background,
                ..appearance
            },
            Button::SelectedPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: palette.normal.primary,
                ..appearance
            }
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let palette = self.palette;

        let background = match style {
            Button::Primary => Some(Background::Color(palette.normal.surface)),
            Button::Unavailable => Some(Background::Color(palette.normal.primary)),
            Button::SelfUpdate => Some(Background::Color(palette.normal.surface)),
            Button::Refresh => Some(Background::Color(palette.normal.surface)),
            Button::UninstallPackage|
            Button::RestorePackage => Some(Background::Color(palette.normal.primary)),
            _ => None
        };

        match style {
            Button::Primary => button::Appearance {
                background: background,
                text_color: palette.bright.primary,
                ..active
            },
            Button::Unavailable => button::Appearance {
                background: background,
                text_color: palette.normal.error,
                ..active
            },
            Button::SelfUpdate => button::Appearance {
                background: background,
                text_color: palette.bright.secondary,
                ..active
            },
            Button::Refresh => button::Appearance {
                background: background,
                text_color: self.palette.bright.primary,
                ..active
            },
            Button::UninstallPackage|
            Button::RestorePackage => button::Appearance {
                background: background,
                text_color: palette.bright.primary,
                ..active
            },
            NormalPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.30,
                    ..palette.normal.primary
                })),
                text_color: palette.bright.primary,
                ..active
            },
            SelectedPackage => button::Appearance { ..active },
        }
    }

    fn disabled(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let palette = self.palette;

        match style {
            RestorePackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.05,
                    ..palette.normal.primary
                })),
                text_color: Color {
                    a: 0.50,
                    ..palette.bright.primary
                },
                ..active
            },
            UninstallPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.01,
                    ..palette.bright.error
                })),
                text_color: Color {
                    a: 0.50,
                    ..palette.bright.error
                },
                ..active
            },
            _ => button::Appearance {
                ..active
            }
        }
    }

    fn pressed(&self, style: Self::Style) -> button::Appearance {
        button::Appearance { ..self.active(style) }
    }
}

impl Default for Scrollable {
    fn default() -> Self {
        Self::Description
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Scrollable {
   Description,
   Packages, 
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: Self::Style) -> scrollable::Scrollbar {
        match style {
            Description => scrollable::Scrollbar {
                background: Some(Background::Color(self.palette.base.foreground)),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: self.palette.base.foreground,
                    border_radius: 0.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
            Packages => scrollable::Scrollbar {
                background: Some(Background::Color(self.palette.base.background)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: self.palette.base.foreground,
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }          
        }
    }

    fn hovered(&self, style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..self.active(style).scroller },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CheckBox {
    PackageEnabled,
    PackageDisabled,
    SettingsEnabled,
    SettingsDisabled
}

impl Default for CheckBox {
    fn default() -> Self {
        Self::PackageEnabled
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: Self::Style, is_checked: bool) -> checkbox::Appearance {        
        match style {
            CheckBox::PackageEnabled=> checkbox::Appearance {
                background: Background::Color(self.palette.base.background),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.base.background,
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::PackageDisabled => checkbox::Appearance {
                background: Background::Color(Color {
                    a: 0.55,
                    ..self.palette.base.background
                }),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: self.palette.normal.primary,
                text_color: Some(self.palette.normal.primary),
            },
            CheckBox::SettingsEnabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.background),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.bright.primary,
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::SettingsDisabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.foreground),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette.normal.primary,
                text_color: Some(self.palette.normal.primary),
            },
        }
    }

    fn hovered(&self, style: Self::Style, is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckBox::PackageEnabled => checkbox::Appearance {
                background: Background::Color(Color {
                    a: 0.5,
                    ..self.palette.bright.primary
                }),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 2.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette.bright.primary
                },
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::SettingsEnabled => checkbox::Appearance {
                background: Background::Color(self.palette.base.foreground),
                checkmark_color: self.palette.bright.primary,
                border_radius: 5.0,
                border_width: 2.0,
                border_color: self.palette.bright.primary,
                text_color: Some(self.palette.bright.surface),
            },
            CheckBox::PackageDisabled => checkbox::Appearance {
                ..self.active(style, is_checked)
            },
            CheckBox::SettingsDisabled => checkbox::Appearance {
                ..self.active(style, is_checked)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextInput {
    Base
}

impl Default for TextInput {
    fn default() -> Self {
        Self::Base
    }
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette.base.foreground),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: self.palette.base.foreground,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette.base.foreground),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette.normal.primary
            },
        }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        self.palette.normal.surface
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        self.palette.bright.primary
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        self.palette.bright.secondary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PickList {
    Base
}

impl Default for PickList {
    fn default() -> Self {
        Self::Base
    }
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> menu::Appearance {
        let palette = self.palette;

        menu::Appearance {
            text_color: palette.base.foreground,
            background: palette.base.background.into(),
            border_width: 1.0,
            border_color: palette.base.background,
            selected_text_color: palette.base.foreground,
            selected_background: palette.base.foreground.into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: ()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette.bright.surface,
            background: self.palette.base.background.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette.normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: self.palette.bright.surface,
        }
    }

    fn hovered(&self, style: ()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            text_color: self.palette.bright.primary,
            ..active
        }
    }
}

#[derive(Clone, Copy)]
pub enum Text {
    Default,
    Color(Color),
    Custom(fn(&Theme) -> text::Appearance),
}

impl Default for Text {
    fn default() -> Self {
        Self::Default
    }
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
            Text::Custom(f) => f(self),
        }
    }
}
