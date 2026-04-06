use crate::settings::ThemeType;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub background: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text: Color,
    #[allow(dead_code)]
    pub brick_palette: [Color; 6],
    pub ball: Color,
    pub paddle: Color,
}

pub fn get_theme_colors(theme: ThemeType) -> ThemeColors {
    match theme {
        ThemeType::Classic => classic_theme(),
        ThemeType::Dark => dark_theme(),
        ThemeType::Neon => neon_theme(),
        ThemeType::Crt => crt_theme(),
        ThemeType::Minimalist => minimalist_theme(),
    }
}

fn classic_theme() -> ThemeColors {
    ThemeColors {
        background: BLACK,
        primary: WHITE,
        secondary: Color::new(0.3, 0.3, 0.3, 1.0),
        accent: Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
        text: WHITE,
        brick_palette: [
            RED,                            // Row 1
            ORANGE,                         // Row 2
            YELLOW,                         // Row 3
            GREEN,                          // Row 4
            Color::new(0.0, 1.0, 1.0, 1.0), // Row 5 - Cyan
            MAGENTA,                        // Row 6
        ],
        ball: Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
        paddle: WHITE,
    }
}

fn dark_theme() -> ThemeColors {
    ThemeColors {
        background: Color::new(0.05, 0.05, 0.08, 1.0),
        primary: Color::new(0.2, 0.2, 0.25, 1.0),
        secondary: Color::new(0.3, 0.3, 0.35, 1.0),
        accent: Color::new(0.3, 0.8, 1.0, 1.0), // Light cyan
        text: Color::new(0.9, 0.9, 0.95, 1.0),
        brick_palette: [
            Color::new(1.0, 0.3, 0.3, 1.0), // Soft red
            Color::new(1.0, 0.6, 0.2, 1.0), // Soft orange
            Color::new(1.0, 0.9, 0.2, 1.0), // Soft yellow
            Color::new(0.3, 1.0, 0.3, 1.0), // Soft green
            Color::new(0.3, 0.8, 1.0, 1.0), // Soft cyan
            Color::new(1.0, 0.4, 1.0, 1.0), // Soft magenta
        ],
        ball: Color::new(0.3, 0.8, 1.0, 1.0),    // Light cyan
        paddle: Color::new(0.9, 0.9, 0.95, 1.0), // Light gray
    }
}

fn neon_theme() -> ThemeColors {
    ThemeColors {
        background: BLACK,
        primary: BLACK,
        secondary: Color::new(0.1, 0.1, 0.1, 1.0),
        accent: Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
        text: Color::new(0.0, 1.0, 1.0, 1.0),   // Cyan
        brick_palette: [
            Color::new(1.0, 0.0, 0.5, 1.0), // Hot pink
            Color::new(1.0, 0.0, 1.0, 1.0), // Magenta
            Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
            Color::new(0.0, 1.0, 0.5, 1.0), // Green-cyan
            Color::new(1.0, 1.0, 0.0, 1.0), // Yellow
            Color::new(1.0, 0.0, 0.0, 1.0), // Red
        ],
        ball: Color::new(0.0, 1.0, 1.0, 1.0),   // Cyan
        paddle: Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
    }
}

fn crt_theme() -> ThemeColors {
    // Retro CRT monitor appearance
    ThemeColors {
        background: BLACK,
        primary: Color::new(0.2, 0.2, 0.15, 1.0),
        secondary: Color::new(0.3, 0.3, 0.25, 1.0),
        accent: Color::new(0.0, 0.8, 0.0, 1.0), // CRT green
        text: Color::new(0.0, 0.8, 0.0, 1.0),   // CRT green
        brick_palette: [
            Color::new(0.8, 0.0, 0.0, 1.0), // Red
            Color::new(0.8, 0.4, 0.0, 1.0), // Orange
            Color::new(0.8, 0.8, 0.0, 1.0), // Yellow
            Color::new(0.0, 0.8, 0.0, 1.0), // Green
            Color::new(0.0, 0.8, 0.8, 1.0), // Cyan
            Color::new(0.8, 0.0, 0.8, 1.0), // Magenta
        ],
        ball: Color::new(0.0, 0.8, 0.0, 1.0),   // Green
        paddle: Color::new(0.0, 0.8, 0.0, 1.0), // Green
    }
}

fn minimalist_theme() -> ThemeColors {
    ThemeColors {
        background: Color::new(0.98, 0.98, 0.98, 1.0), // Off-white
        primary: Color::new(0.9, 0.9, 0.9, 1.0),
        secondary: Color::new(0.8, 0.8, 0.8, 1.0),
        accent: Color::new(0.2, 0.2, 0.8, 1.0), // Navy blue
        text: Color::new(0.1, 0.1, 0.1, 1.0),   // Dark text
        brick_palette: [
            Color::new(0.8, 0.2, 0.2, 1.0), // Muted red
            Color::new(0.8, 0.5, 0.2, 1.0), // Muted orange
            Color::new(0.8, 0.8, 0.2, 1.0), // Muted yellow
            Color::new(0.2, 0.8, 0.2, 1.0), // Muted green
            Color::new(0.2, 0.8, 0.8, 1.0), // Muted cyan
            Color::new(0.8, 0.2, 0.8, 1.0), // Muted magenta
        ],
        ball: Color::new(0.2, 0.2, 0.8, 1.0),   // Navy blue
        paddle: Color::new(0.1, 0.1, 0.1, 1.0), // Dark gray
    }
}
