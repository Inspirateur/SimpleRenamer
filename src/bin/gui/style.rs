use dark_light;
use iced::{button, container, scrollable, scrollable::Scroller, text_input, Background, Color};

#[derive(Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub font: Color,
    pub font_shade: Color,
    pub greyed: Color,
    pub hint: Color,
    pub row_even: Color,
    pub row_odd: Color,
    pub button: Color,
    pub button_shade: Color,
    pub accent: Color,
    pub accent_shade: Color,
    pub error: Color,
}

impl Palette {
    fn new() -> Self {
        // light theme by default, dark theme only if detected
        if dark_light::detect() == dark_light::Mode::Dark {
            Palette {
                background: Color::from_rgb(0.1, 0.1, 0.1),
                font: Color::from_rgb(0.8, 0.8, 0.8),
                font_shade: Color::from_rgb(0.7, 0.7, 0.7),
                greyed: Color::from_rgb(0.5, 0.5, 0.5),
                hint: Color::from_rgb(0.3, 0.3, 0.3),
                row_even: Color::from_rgba(0.13, 0.14, 0.14, 0.7),
                row_odd: Color::from_rgba(0.17, 0.18, 0.18, 0.7),
                button: Color::from_rgb(0.18, 0.18, 0.18),
                button_shade: Color::from_rgb(0.22, 0.22, 0.22),
                accent: Color::from_rgb(0.35, 0.50, 0.75),
                accent_shade: Color::from_rgb(0.4, 0.55, 0.8),
                error: Color::from_rgb(0.9, 0.4, 0.5),
            }
        } else {
            Palette {
                background: Color::WHITE,
                font: Color::from_rgb(0.1, 0.1, 0.1),
                font_shade: Color::from_rgb(0.2, 0.2, 0.2),
                greyed: Color::from_rgb(0.6, 0.56, 0.56),
                hint: Color::from_rgb(0.8, 0.8, 0.8),
                row_even: Color::from_rgba(0.86, 0.92, 1., 0.7),
                row_odd: Color::from_rgba(0.92, 0.96, 1., 0.7),
                button: Color::from_rgb(0.96, 0.96, 0.96),
                button_shade: Color::from_rgb(0.86, 0.86, 0.86),
                accent: Color::from_rgb(0.1, 0.41, 0.86),
                accent_shade: Color::from_rgb(0.05, 0.3, 0.8),
                error: Color::from_rgb(0.7, 0.1, 0.2),
            }
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

impl container::StyleSheet for Palette {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.background.into(),
            ..Default::default()
        }
    }
}

impl button::StyleSheet for Palette {
    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.hint)),
            text_color: self.greyed,
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.button)),
            text_color: self.font,
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.button_shade)),
            text_color: self.font,
            border_radius: 2.0,
            ..Default::default()
        }
    }
}

impl text_input::StyleSheet for Palette {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background),
            border_color: self.hint,
            border_width: 1.0,
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background),
            border_color: self.accent,
            border_width: 1.0,
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn placeholder_color(&self) -> Color {
        self.hint
    }

    fn value_color(&self) -> Color {
        self.font
    }

    fn selection_color(&self) -> Color {
        self.hint
    }
}

impl scrollable::StyleSheet for Palette {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.background)),
            border_radius: 0.,
            border_width: 0.,
            border_color: Color::TRANSPARENT,
            scroller: Scroller {
                color: self.hint,
                border_radius: 4.,
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(self.background)),
            border_radius: 0.,
            border_width: 0.,
            border_color: Color::TRANSPARENT,
            scroller: Scroller {
                color: self.greyed,
                border_radius: 4.,
                border_width: 0.,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
pub struct RowEven(pub Palette);

impl container::StyleSheet for RowEven {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.0.row_even.into(),
            ..Default::default()
        }
    }
}

pub struct RowOdd(pub Palette);

impl container::StyleSheet for RowOdd {
    fn style(&self) -> container::Style {
        container::Style {
            background: self.0.row_odd.into(),
            ..Default::default()
        }
    }
}

pub struct ApplyButton(pub Palette);

impl button::StyleSheet for ApplyButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.0.accent)),
            text_color: Color::WHITE,
            border_radius: 2.0,
            ..Default::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.0.accent_shade)),
            text_color: Color::WHITE,
            border_radius: 2.0,
            ..Default::default()
        }
    }
}
