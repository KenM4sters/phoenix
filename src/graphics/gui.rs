use egui_winit::State;
use wgpu::{CommandEncoder, Device, TextureFormat, TextureView}; // Import wgpu types directly
use egui::{epaint::Shadow, Align2, Context, Visuals};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use winit::window::Window;

pub struct Gui {
    pub ctx: Context,
    state: State,
    renderer: Renderer,
}

impl Gui {
    pub fn new(device: &Device, target_color_format: TextureFormat, target_depth_format: Option<TextureFormat>, window: &Window) -> Self {
        let ctx = Context::default();
        let id = ctx.viewport_id();

        const BORDER_RADIUS: f32 = 2.0;

        let visuals = Visuals {
            window_rounding: egui::Rounding::same(BORDER_RADIUS),
            window_shadow: Shadow::NONE,
            ..Default::default()
        };

        ctx.set_visuals(visuals);

        let state = egui_winit::State::new(ctx.clone(), id, &window, None, None);

        let renderer = Renderer::new(device, target_color_format, target_depth_format, 1);

        Self {
            ctx,
            state,
            renderer,
        }
    }

    pub fn draw(
        &mut self,
        device: &super::context::Device,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        run_gui: impl FnOnce(&Context)
    ) {

        let queue = &device.queue;
        let logical_device = &device.logical_device;

        // self.state.set_pixels_per_point(window.scale_factor() as f32);
        let raw_input = self.state.take_egui_input(&window);

        let full_output = self.ctx.run(raw_input, |ui| {
            run_gui(&self.ctx);
        });

        self.state.handle_platform_output(&window, full_output.platform_output);

        let tris = self.ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        full_output.textures_delta.set.iter().for_each(|(id, image_delta)| {
            self.renderer.update_texture(&logical_device, &queue, *id, &image_delta);
        });

        self.renderer.update_buffers(&logical_device, &queue, encoder, &tris, &screen_descriptor);
        
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &window_surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                label: Some("egui main render pass"),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.render(&mut rpass, &tris, &screen_descriptor);
        }
        
        full_output.textures_delta.set.iter().for_each(|(id, image_delta)| {
            self.renderer.free_texture(id);
        });
    }
}

pub fn example_gui(ui: &Context) {
    egui::Window::new("Streamline CFD")
        // .vscroll(true)
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, |ui| {
            if ui.add(egui::Button::new("Click me")).clicked() {
                println!("PRESSED")
            }

            ui.label("Slider");
            // ui.add(egui::Slider::new(_, 0..=120).text("age"));
            ui.end_row();

            // proto_scene.egui(ui);
        });
}