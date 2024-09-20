use crossterm::event::{
    KeyCode::{Down, End, Home, Left, PageDown, PageUp, Right, Left},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]

pub enum Move{
    PageDown,
    PageUp,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}

impl TryFrom<KeyEvent> for Move{
    type Error = String;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent{
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageDown => Ok(Self::PageDown),
                PageUp => Ok(Self::PageUp),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}