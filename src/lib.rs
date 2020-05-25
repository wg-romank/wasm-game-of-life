use std::collections::HashSet;

use wasm_bindgen::prelude::*;

use glsmrs as gl;

mod universe;
mod shaders;

const CELL_SIZE: u32 = 5;


#[wasm_bindgen]
pub fn get_cell_size() -> Result<u32, JsValue> { Ok(CELL_SIZE) }

#[wasm_bindgen]
pub fn setup_canvas(universe: &universe::Universe) -> Result<(), JsValue> {
    let canvas = shaders::get_canvas().ok_or(JsValue::from_str("Failed getting canvas"))?;
    canvas.set_width((CELL_SIZE + 1) * universe.width() + 1);
    canvas.set_height((CELL_SIZE + 1) * universe.height() + 1);

    Ok(())
}

#[wasm_bindgen]
pub fn setup_webgl() -> Result<gl::GlState, JsValue> {
    shaders::setup_shaders()
}

fn compute_draw_cells_webgl(changes: &HashSet<(u32, u32)>) -> (Vec<f32>, Vec<u16>) {
    let mut vertexes = Vec::new();
    let mut indices = Vec::new();
    let mut current_idx = 0;
    let fcs = CELL_SIZE as f32;

    for &(row, col) in changes {
        let scaled = |idx: u32| { (idx as f32) * (fcs + 1.) + 1. };

        let v0 = vec![
            scaled(col), scaled(row),
            scaled(col) + fcs, scaled(row),
            scaled(col) + fcs, scaled(row) + fcs,
            scaled(col), scaled(row) + fcs
        ];

        v0.into_iter().for_each(|v| vertexes.push(v));

        let e1 = vec![
            current_idx, current_idx + 1, current_idx + 2,
            current_idx + 2, current_idx + 3, current_idx
        ];

        e1.into_iter().for_each(|e| indices.push(e));
        
        // todo ???
        current_idx += 6
    }

    (vertexes, indices)
}

#[wasm_bindgen]
pub fn animation_webgl(state: &mut gl::GlState, universe: &mut universe::Universe, ticks: u32) -> Result<(), JsValue> {
    universe.tick_many(ticks);
    let (vertices, indices) = compute_draw_cells_webgl(universe.alive_cells());
    shaders::render_pipeline(state, &vertices, &indices)
}
