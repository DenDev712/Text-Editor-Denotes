use crossterm::style::Color;
use super::super::AnnotationType;
pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 100,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 126,
                    g: 43,
                    b: 234,
                }),
            },

            AnnotationType::Digit => Self{
                foreground: Some(Color:: Rgb { 
                    r: 100, 
                    g: 100, 
                    b: 100,
                }),
                background: None,
            },
        }
    }
}