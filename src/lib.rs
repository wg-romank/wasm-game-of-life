use wasm_bindgen::prelude::*;

use glsmrs as gl;

mod shaders;

#[wasm_bindgen]
pub fn setup_webgl() -> Result<gl::GlState, JsValue> {
    shaders::setup_shaders()
}

#[wasm_bindgen]
pub fn setup_init_program() -> Result<gl::Program, JsValue> {
    shaders::setup_display_program()
}

#[wasm_bindgen]
pub fn setup_compute_program() -> Result<gl::Program, JsValue> {
    shaders::setup_compute_program()
}

#[wasm_bindgen]
pub fn setup_copy_program() -> Result<gl::Program, JsValue> {
    shaders::setup_copy_program()
}

#[wasm_bindgen]
pub fn animation_webgl(
    program: &gl::Program,
    compute_program: &gl::Program,
    copy_program: &gl::Program,
    state: &mut gl::GlState
) -> Result<(), JsValue> {
    shaders::render_pipeline(program, compute_program, copy_program, state)
}
