use winit::{event::WindowEvent, event_loop::EventLoopWindowTarget};



pub struct Controller {
    input_callback: fn(event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>)
}

impl Controller {
    pub fn new(input_callback: fn(event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>)) -> Self {
        Self {
            input_callback
        }
    }

    pub fn call(&self, event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
        (self.input_callback)(event, control_flow);
    }
}

