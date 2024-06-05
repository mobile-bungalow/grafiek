mod init;

use std::collections::HashMap;

use grafiek_engine::{
    document::{self, Document},
    Engine,
};
use log::info;
use log::Level;
use tweak_shader::RenderContext;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::{Surface, SurfaceTarget};

const FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

#[wasm_bindgen]
pub struct EngineWrapper {
    device: wgpu::Device,
    queue: wgpu::Queue,
    instance: wgpu::Instance,
    letter_box_ctx: RenderContext,
    engine: Engine,
    node_surfaces: HashMap<usize, Surface<'static>>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum EdgeType {
    Image,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct EdgeInfo {
    pub source_node_id: usize,
    pub sync_node_id: usize,
    pub source_arg_idx: usize,
    pub sync_arg_idx: usize,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub label: String,
    pub id: usize,
    pub ty: NodeTypes,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum NodeTypes {
    Input = "Input",
    Output = "Output",
    GrayScale = "GrayScale",
}

impl From<document::Op> for NodeTypes {
    fn from(value: document::Op) -> Self {
        match value {
            document::Op::GrayScale => NodeTypes::GrayScale,
            document::Op::Input { .. } => NodeTypes::Input,
            document::Op::Output { .. } => NodeTypes::Output,
        }
    }
}

#[wasm_bindgen]
impl EngineWrapper {
    pub async fn init(json: String) -> Result<EngineWrapper, String> {
        console_log::init_with_level(Level::Debug).map_err(|_| "log init failed".to_owned())?;
        let init::WgpuContext {
            queue,
            device,
            instance,
            ..
        } = init::new_ctx().await?;
        let doc = Document::load(json).map_err(|e| format!("{e:?}"))?;
        let engine = Engine::new(&doc, &device, &queue).map_err(|e| format!("{e:?}"))?;

        let letter_box_ctx = RenderContext::new(
            include_str!("../resources/letterbox.fs"),
            FMT,
            &device,
            &queue,
        )
        .map_err(|_| "Could not init letterbox ctx".to_owned())?;

        Ok(EngineWrapper {
            engine,
            queue,
            device,
            letter_box_ctx,
            instance,
            node_surfaces: HashMap::new(),
        })
    }

    pub fn render(&mut self) {
        self.engine.render(&self.device, &self.queue);
        self.update_previews();
    }

    pub fn list_nodes(&self) -> Vec<NodeInfo> {
        self.engine
            .iter_nodes()
            .map(|n| NodeInfo {
                label: n.label.to_owned(),
                id: n.id,
                ty: n.op_type.into(),
            })
            .collect()
    }

    pub fn list_edges(&self) -> Vec<EdgeInfo> {
        self.engine
            .iter_edges()
            .map(|e| EdgeInfo {
                source_node_id: e.source_node,
                sync_node_id: e.sync_node,
                source_arg_idx: e.source_arg_index,
                sync_arg_idx: e.sync_arg_index,
            })
            .collect()
    }

    pub fn remove_node(&mut self, id: usize) {}

    pub fn register_surface(&mut self, id: usize, canvas: HtmlCanvasElement) {
        let wgpu_surface = self
            .instance
            .create_surface(SurfaceTarget::Canvas(canvas))
            .unwrap();

        self.node_surfaces.insert(id, wgpu_surface);
    }

    pub fn update_node_metadata(&mut self, id: usize, metadata: usize) {}

    pub fn connect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {}

    pub fn disconnect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {
    }

    pub fn update_previews(&mut self) {
        for (id, surface) in self.node_surfaces.iter() {
            info!("updating previews!");
            let Some(preview) = self.engine.get_preview(*id as u32) else {
                info!("continuing!");
                continue;
            };

            let surface = match surface.get_current_texture() {
                Ok(s) => s,
                Err(e) => {
                    info!("{e:?}");
                    continue
                }
            };

           // info!("unwrap!");
           // let width = surface.texture.width() as f32;
           // let height = surface.texture.height() as f32;

           // *self
           //     .letter_box_ctx
           //     .get_input_as::<f32>("output_height")
           //     .unwrap() = height;
           // *self
           //     .letter_box_ctx
           //     .get_input_as::<f32>("output_width")
           //     .unwrap() = width;
           // *self
           //     .letter_box_ctx
           //     .get_input_as::<f32>("aspect_ratio")
           //     .unwrap() = width / height;
           // self.letter_box_ctx
           //     .load_shared_texture(preview, "input_image");
           // info!("unwrapped all!");

           // self.letter_box_ctx.render(
           //     &self.queue,
           //     &self.device,
           //     &surface.texture.create_view(&Default::default()),
           //     surface.texture.width(),
           //     surface.texture.height(),
           // );
           // info!("render!");

           // surface.present();
            info!("present!");
        }
        info!("done!");
    }

    pub fn set_input() {}

    pub fn export_output(&self, name: &str) {}
}
