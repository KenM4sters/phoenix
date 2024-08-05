
use winit::{
    window::Window,
};

use crate::graphics::Graphics;

struct Engine<'a> {
    window: Window,
    graphics: Graphics<'a>,
}


trait EngineLayer {
    fn resize(physical_size: winit::dpi::PhysicalSize<u32>) -> Result<(), ()>;
}