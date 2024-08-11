use winit::event::WindowEvent;

trait GameLayer {
    fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>);

    fn handle_window_input(&self, event: Option<&WindowEvent>);
}

pub struct Input<'a> {
    listeners: Vec<&'a dyn GameLayer>
}

impl<'a> Input<'a> {
    pub fn new() -> Self {
        Self {
            listeners: vec![]
        }
    }

    pub fn add_listener(&mut self, listener: &'a dyn GameLayer) {
        self.listeners.push(listener);
    }

    pub fn call_listeners(&self, window_event: &WindowEvent) {
        self.listeners
            .iter()
            .for_each(move |listener| {
                listener.handle_window_input(Some(window_event))
            });
    }
}