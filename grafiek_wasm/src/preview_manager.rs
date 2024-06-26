use std::collections::HashMap;
use std::collections::HashSet;

use tweak_shader::RenderContext;
use wgpu::Device;
use wgpu::Queue;
use wgpu::Texture;

use log::info;

struct SurfaceMeta {
    pub surface: wgpu::Surface<'static>,
    pub cleared: bool,
}

// Manages the surfaces for every nodes preview window
pub struct PreviewManager {
    // the windows which are off screen, or which the user has closed to save fillrate
    hidden: HashSet<usize>,
    surfaces: HashMap<usize, SurfaceMeta>,
    // a downsampler that maintains the aspect ratio of the input image
    // by inserting black bars to pad width and height.
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

    pub fn remove_surface(&mut self, id: usize) {
        self.hidden.remove(&id);
        self.surfaces.remove(&id);
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

        self.surfaces.insert(
            id,
            SurfaceMeta {
                cleared: false,
                surface: wgpu_surface,
            },
        );
    }

    pub fn clear(&mut self, device: &Device, queue: &Queue, id: usize) {
        let Some(SurfaceMeta { cleared, surface }) = self.surfaces.get_mut(&id) else {
            return;
        };

        if *cleared {
            return;
        }

        let surface = match surface.get_current_texture() {
            Ok(s) => s,
            Err(e) => {
                info!("{e:?}");
                return;
            }
        };

        let view = surface.texture.create_view(&Default::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Clear Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: Default::default(),
            });
        }

        queue.submit(std::iter::once(encoder.finish()));
        surface.present();

        *cleared = true;
    }

    pub fn update_preview(&mut self, device: &Device, queue: &Queue, id: usize, preview: &Texture) {
        let Some(SurfaceMeta { surface, cleared }) = self.surfaces.get_mut(&id) else {
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
        *cleared = false;

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
