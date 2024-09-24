use std::{io::Error, time::Duration, time::Instant};

use super::super::{Size, Terminal};
use super::UIComponent;
//the message duration before hiding
const DEFAULT_DURATION: Duration = Duration::new(5, 0);
struct Message{
    text: String,
    time: Instant,    
}

impl Default for Message{
    fn default() -> Self {
        Self{
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message{
    fn has_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}
#[derive(Default)]
pub struct MessageBar{
    current_message: Message,
    needs_redraw: bool,
    cleared_expired_message: bool,
}

impl MessageBar{
    pub fn update_message(&mut self, new_message: &str){
        self.current_message = Message{
            text: new_message.to_string(),
            time: Instant::now(),
        };

        self.cleared_expired_message = false;
        self.mark_redraw(true);
    }
}

impl UIComponent for MessageBar{
    fn mark_redraw(&mut self, value: bool){
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool{
        (!self.cleared_expired_message && self.current_message.has_expired()) || 
        self.needs_redraw
    }

    fn set_size(&mut self, _: Size){}

    fn draw(&mut self, origin: usize) -> Result<(), Error>{
        if self.current_message.has_expired(){
            self.cleared_expired_message = true;
        }
        let message = if self.current_message.has_expired() {
            ""
        }else{
            &self.current_message.text
        };
        Terminal::print_row(origin, message)
    }
}