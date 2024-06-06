use wasm_bindgen::prelude::*;
use grafiek_engine::document;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum NodeTypes {
    Input = "Input",
    Output = "Output",
    GrayScale = "GrayScale",
    Shader = "Shader",
}

impl From<document::Op> for NodeTypes {
    fn from(value: document::Op) -> Self {
        match value {
            document::Op::GrayScale => NodeTypes::GrayScale,
            document::Op::Input { .. } => NodeTypes::Input,
            document::Op::Output { .. } => NodeTypes::Output,
            document::Op::CustomShader { .. } => NodeTypes::Shader,
        }
    }
}