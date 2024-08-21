use std::io::Error;

use super::terminal::Size;
pub trait UIComponent{
    //marks this component if it needs redrawing or not
    fn mark_redraw(&mut self, value: bool);
    //determines if the component actually needs redraw or not
    fn needs_redraw(&self) -> bool;


    //updates the size and marks the said component as it needs to be redrawn
    fn resize(&mut self, size: Size){
        self.set_size(size);
        self.mark_redraw(true);
    }

    //updates the size
    fn set_size(&mut self, size: Size);

    //drawing this component if its visible and needs to be redrawn
    fn render(&mut self, origin_y: usize){
        if self.needs_redraw(){
            match self.draw(origin_y){
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }
    fn draw(&mut self, origin_y: usize) -> Result<(), Error>;
}