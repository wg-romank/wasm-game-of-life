use wasm_bindgen::prelude::*;

use glsmrs as gl;

mod shaders;

const CELL_SIZE: u32 = 5;

#[wasm_bindgen]
pub fn get_cell_size() -> Result<u32, JsValue> { Ok(CELL_SIZE) }

#[wasm_bindgen]
pub fn setup_canvas() -> Result<(), JsValue> {
    let canvas = shaders::get_canvas().ok_or(JsValue::from_str("Failed getting canvas"))?;
    canvas.set_width(64);
    canvas.set_height(64);
    // canvas.set_width((CELL_SIZE + 1) * universe.width() + 1);
    // canvas.set_height((CELL_SIZE + 1) * universe.height() + 1);

    Ok(())
}


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
pub fn animation_webgl(
    program: &gl::Program,
    compute_program: &gl::Program,
    state: &mut gl::GlState
) -> Result<(), JsValue> {
    shaders::render_pipeline(program, compute_program, state)
}
