use std::collections::HashMap;
use std::collections::HashSet;

use tweak_shader::RenderContext;
use wgpu::Device;
use wgpu::Queue;
use wgpu::Texture;

use log::info;

pub struct PreviewManager {
    hidden: HashSet<usize>,
    surfaces: HashMap<usize, wgpu::Surface<'static>>,
    letterbox: RenderContext,
}

impl PreviewManager {
    pub fn new(device: &Device, queue: &Queue) -> Result<Self, String> {
        let letterbox = RenderContext::new(
            include_str!("../resources/letterbox.fs"),
            crate::FMT,
            &device,
            &queue,
        )
        .map_err(|_| "Could not init letterbox ctx".to_owned())?;

        Ok(Self {
            hidden: HashSet::new(),
            surfaces: HashMap::new(),
            letterbox,
        })
    }

    pub fn register_surface(
        &mut self,
        id: usize,
        canvas: web_sys::HtmlCanvasElement,
        device: &Device,
        instance: &wgpu::Instance,
    ) {
        let width = canvas.width();
        let height = canvas.height();

        let wgpu_surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .unwrap();

        wgpu_surface.configure(
            device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: crate::FMT,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
                view_formats: vec![crate::FMT],
            },
        );

        self.surfaces.insert(id, wgpu_surface);
    }

    pub fn update_preview(&mut self, device: &Device, queue: &Queue, id: usize, preview: &Texture) {
        let Some(surface) = self.surfaces.get(&id) else {
            return;
        };

        let surface = match surface.get_current_texture() {
            Ok(s) => s,
            Err(e) => {
                info!("{e:?}");
                return;
            }
        };

        let width = surface.texture.width() as f32;
        let height = surface.texture.height() as f32;
        let view = &surface.texture.create_view(&Default::default());

        *self.letterbox.get_input_as::<f32>("output_height").unwrap() = height;
        *self.letterbox.get_input_as::<f32>("output_width").unwrap() = width;
        *self.letterbox.get_input_as::<f32>("aspect_ratio").unwrap() = width / height;
        self.letterbox.load_shared_texture(preview, "image");
        self.letterbox
            .render(&queue, &device, view, width as u32, height as u32);

        surface.present();
    }

    pub fn dirty_nodes(&self) -> Vec<usize> {
        self.surfaces
            .keys()
            .filter(|k| !self.hidden.contains(*k))
            .cloned()
            .collect()
    }
}
