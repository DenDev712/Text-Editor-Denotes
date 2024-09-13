use std::io::Error;

use super::Size;
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
    fn render(&mut self, origin_row: usize){
        if self.needs_redraw(){
            if let Err(err) = self.draw(origin_row) {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not render component: {err:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = err;
                }
            } else {
                self.mark_redraw(false);
            }
        }
    }
    fn draw(&mut self, origin_row: usize) -> Result<(), Error>;
}