mod attribute;
use attribute::Attribute;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{
    Attribute::{Reset, Reverse},
    Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen, DisableLineWrap, EnableLineWrap, SetTitle
};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

use super::AnnotatedString;
use super::{Position, Size};
/// Represents the Terminal.
pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        Self::enable_line_wrap()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    /// Moves the caret to the given Position.
    
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { row, col: 0 })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    pub fn print_annotated_row(
                row: usize,
                annotated_string: &AnnotatedString,
            ) -> Result<(), Error> {
                Self::move_caret_to(Position { row, col: 0 })?;
                Self::clear_line()?;
                annotated_string
                    .into_iter()
                    .try_for_each(|part| -> Result<(), Error> {
                        if let Some(annotation_type) = part.annotation_type {
                            let attribute: Attribute = annotation_type.into();
                            Self::set_attribute(&attribute)?;
                        }
                        Self::print(part.string)?;
                        Self::reset_color()?;
                        Ok(())
                    })?;
                Ok(())
            }
            fn set_attribute(attribute: &Attribute) -> Result<(), Error> {
                if let Some(foreground_color) = attribute.foreground {
                    Self::queue_command(SetForegroundColor(foreground_color))?;
                }
                if let Some(background_color) = attribute.background {
                    Self::queue_command(SetBackgroundColor(background_color))?;
                }
                Ok(())
            }
            fn reset_color() -> Result<(), Error> {
                Self::queue_command(ResetColor)?;
                Ok(())
            }
    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(row, &format!("{Reverse}{line_text:width$.width$}{Reset}"))
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}