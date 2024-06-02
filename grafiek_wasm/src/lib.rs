use grafiek_engine::Engine;
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
    ty: EdgeType,
    label: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub label: String,
    pub inputs: Vec<EdgeInfo>,
    pub output_info: Vec<EdgeInfo>,
    pub config: Vec<EdgeInfo>,
    pub index: usize,
}

#[wasm_bindgen]
impl EngineWrapper {
    pub fn init(json: String) -> Option<EngineWrapper> {
        None
    }

    pub fn list_nodes(&self) -> Vec<NodeInfo> {
        vec![]
    }

    pub fn list_edges(&self) -> Vec<NodeInfo> {
        vec![]
    }

   // pub fn add_node(&mut self, optype: usize, label: &str ) -> NodeInfo {
   //     todo!()
   // }

    pub fn remove_node(&mut self, id: usize) {}

    /// Change node position before save
    pub fn update_node_metadata(&mut self, id: usize, metadata: usize) {}

    pub fn connect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {}

    pub fn disconnect_nodes(&mut self, out_id: usize, in_id: usize, out_edge: &str, in_edge: &str) {}

    pub fn export_output(&self, name: &str) {}
}
