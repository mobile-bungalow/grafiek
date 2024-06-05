mod init;
mod preview_manager;

use grafiek_engine::{
    document::{self, Document},
    Engine,
};
use preview_manager::PreviewManager;

// wasm
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

// logging
use console_error_panic_hook;
use log::Level;
use std::panic;

const FMT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

#[wasm_bindgen]
pub struct EngineWrapper {
    device: wgpu::Device,
    queue: wgpu::Queue,
    instance: wgpu::Instance,
    engine: Engine,
    preview_manager: PreviewManager,
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
        console_error_panic_hook::set_once();
        console_log::init_with_level(Level::Debug).map_err(|_| "log init failed".to_owned())?;
        let init::WgpuContext {
            queue,
            device,
            instance,
            ..
        } = init::new_ctx().await?;
        let doc = Document::load(json).map_err(|e| format!("{e:?}"))?;
        let engine = Engine::new(&doc, &device, &queue).map_err(|e| format!("{e:?}"))?;

        let preview_manager = PreviewManager::new(&device, &queue)?;
        Ok(EngineWrapper {
            engine,
            queue,
            device,
            instance,
            preview_manager,
        })
    }

    pub fn render(&mut self) {
        self.engine.render(&self.device, &self.queue);
        self.update_all_previews();
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
        self.preview_manager
            .register_surface(id, canvas, &self.device, &self.instance)
    }

    pub fn update_node_metadata(&mut self, id: usize, metadata: usize) {}

    pub fn connect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {}

    pub fn disconnect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {
    }

    /// TODO: more info logged on failure / throw an error
    pub fn update_preview(&mut self, id: usize) {
        let Some(preview) = self.engine.get_preview(id as u32) else {
            return;
        };

        self.preview_manager
            .update_preview(&self.device, &self.queue, id, preview);
    }

    pub fn update_all_previews(&mut self) {
        let dirty_nodes = self.preview_manager.dirty_nodes();
        dirty_nodes.into_iter().for_each(|id| self.update_preview(id));
    }

    //TODO: get rid of this and generalize
    pub fn set_input_image(&mut self, data: Vec<u8>, width: u32, height: u32, id: usize) {
        
        self.engine
            .set_input_var("input", data, width, height, &self.device, &self.queue);
    }

    pub fn export_output(&self, name: &str) {}
}
