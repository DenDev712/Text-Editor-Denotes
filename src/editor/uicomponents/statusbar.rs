use std::io::Error;

use super::super::{DocumentStatus, Size, Terminal};
use super::UIComponent;

#[derive(Default)]
pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl StatusBar {
    
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for StatusBar{
    fn mark_redraw(&mut self, value: bool){
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool{
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size){
        self.size = size;
    }

    fn draw(&mut self, origin_row: usize) -> Result<(), Error>{
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        
        let begin = format!(
            "{} - {line_count} {modified_indicator}",
            self.current_status.file_name
        );
        
        let position_indicator = self.current_status.position_indicator_to_string();

        let file_type = self.current_status.file_type_to_string();
        let back_part = format!("{file_type} | {position_indicator}");

        let remain_len = self.size.width.saturating_sub(begin.len());
        let status = format!("{begin}{back_part:>remain_len$}");

        //will only print the status if it fits
        let to_print = if status.len() <= self.size.width{
            status
        }else{
            //else will return an empty string
            String::new()
        };
        Terminal::print_inverted_row(origin_row, &to_print)?;

        Ok(())
    }
}