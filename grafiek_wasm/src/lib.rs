mod init;
mod node_types;
mod preview_manager;
mod util;

use grafiek_engine::{
    document::{self, Document, Op},
    Engine, TryAsRef,
};
use node_types::NodeTypes;
use preview_manager::PreviewManager;

// wasm
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

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
#[wasm_bindgen(getter_with_clone)]
pub struct EdgeInfo {
    pub source_node_id: usize,
    pub sync_node_id: usize,
    pub source_arg_idx: usize,
    pub sync_arg_idx: usize,
    pub id: usize,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub label: String,
    pub id: usize,
    pub ty: NodeTypes,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[wasm_bindgen]
impl EngineWrapper {
    pub async fn init(json: String) -> Result<EngineWrapper, String> {
        console_error_panic_hook::set_once();

        let level = if cfg!(debug_assertions) {
            log::Level::Trace
        } else {
            log::Level::Info
        };
        console_log::init_with_level(level).map_err(|_| "log init failed".to_owned())?;

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
                id: e.id,
                //TODO: i maybe yagni'd these indices :/
                source_node_id: e.source_node,
                sync_node_id: e.sync_node,
                source_arg_idx: e.source_arg_index,
                sync_arg_idx: e.sync_arg_index,
            })
            .collect()
    }

    pub fn remove_node(&mut self, id: usize) {
        self.engine.remove_node(id as u32);
        self.preview_manager.remove_surface(id);
    }

    pub fn remove_edge(&mut self, edge_id: usize) {
        self.engine.remove_edge(edge_id as u32);
    }

    pub fn register_surface(&mut self, id: usize, canvas: HtmlCanvasElement) {
        self.preview_manager
            .register_surface(id, canvas, &self.device, &self.instance)
    }

    pub fn connect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: usize, in_edge: usize) {
        self.engine
            .connect_nodes(out_id as u32, in_id as u32, out_edge, in_edge);
    }

    // TODO: it seems like each type will need it's own constructor, for now we just need to steel cable with
    // grayscale
    pub fn add_node(&mut self, label: String) -> usize {
        let desc = document::NodeDesc {
            op: Op::GrayScale,
            info: document::NodeInfo {
                label: label.into(),
            },
        };
        let idx = self.engine
            .add_node(desc, &self.device, &self.queue)
            .unwrap();

        idx.index()
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
        dirty_nodes
            .into_iter()
            .for_each(|id| self.update_preview(id));
    }

    //TODO: get rid of this and generalize
    pub fn set_input_image(&mut self, data: Vec<u8>, width: u32, height: u32, id: usize) {
        self.engine
            .set_input_var("input", data, width, height, &self.device, &self.queue);
    }

    pub async fn export_image_output(&self, name: &str) -> Result<ImageInfo, String> {
        if let Some(tex) = self.engine.get_output(name).and_then(|t| t.try_as_ref()) {
            let vec = util::read_texture_contents_to_vec(&self.device, &self.queue, tex).await?;

            Ok(ImageInfo {
                data: vec,
                height: tex.height(),
                width: tex.width(),
            })
        } else {
            Err("no output of that name".to_owned())
        }
    }
}
