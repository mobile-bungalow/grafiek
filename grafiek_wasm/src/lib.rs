mod init;

use grafiek_engine::{document::Document, Engine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EngineWrapper {
    device: wgpu::Device,
    queue: wgpu::Queue,
    engine: Engine,
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
    //pub inputs: Vec<EdgeInfo>,
    //pub output_info: Vec<EdgeInfo>,
    //pub config: Vec<EdgeInfo>,
}

#[wasm_bindgen]
impl EngineWrapper {
    pub async fn init(json: String) -> Result<EngineWrapper, String> {
        let init::WgpuContext { queue, device, .. } = init::new_ctx().await?;
        let doc = Document::load(json).map_err(|e| format!("{e:?}"))?;
        let engine = Engine::new(&doc, &device, &queue).map_err(|e| format!("{e:?}"))?;
        Ok(EngineWrapper {
            engine,
            queue,
            device,
        })
    }

    pub fn list_nodes(&self) -> Vec<NodeInfo> {
        self.engine
            .iter_nodes()
            .map(|n| NodeInfo {
                label: n.label.to_owned(),
                id: n.id,
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

    /// Change node position before save
    pub fn update_node_metadata(&mut self, id: usize, metadata: usize) {}

    pub fn connect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {}

    pub fn disconnect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {
    }

    pub fn export_output(&self, name: &str) {}
}
