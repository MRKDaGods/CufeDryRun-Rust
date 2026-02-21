pub mod main_window;
pub use main_window::MainWindow;

use crate::{views::View};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait Window {
    fn views(&self) -> &HashMap<TypeId, Box<dyn View>>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
