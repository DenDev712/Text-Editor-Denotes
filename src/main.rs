#![warn(
    clippy::all, 
    clippy::pedantic, 
    clippy::print_stdout, 
    clippy::arithmetic_side_effects, 
    clippy::as_conversions, 
    clippy::integer_division
)]

mod editor;
use editor::Editor;
mod prelude;
fn main(){
    // calling a method on editor
    Editor::new().unwrap().run(); 
}



    
