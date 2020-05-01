use std::collections::HashSet;

use wasm_bindgen::prelude::*;

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
pub fn setup_webgl() -> Result<(), JsValue> {
    shaders::setup_shaders()?;

    Ok(())
}

fn compute_draw_cells_webgl(changes: &HashSet<(u32, u32)>) -> Vec<f32> {
    let mut vertexes = Vec::new();
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
    }

    vertexes
}

#[wasm_bindgen]
pub fn animation_webgl(universe: &mut universe::Universe, ticks: u32) -> Result<(), JsValue> {
    universe.tick_many(ticks);
    let vertices = compute_draw_cells_webgl(universe.alive_cells());
    shaders::render_pipeline(&vertices, 8)  // 8 vertices per quad
}
