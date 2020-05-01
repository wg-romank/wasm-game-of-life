use std::collections::HashSet;

use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext as WebGl};

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
pub fn setup_webgl() -> Result<(), JsValue> {
    shaders::setup_shaders()?;

    Ok(())
}

#[wasm_bindgen]
pub fn animation_webgl(universe: &mut universe::Universe, ticks: u32) -> Result<(), JsValue> {
    universe.tick_many(ticks);

    let context: WebGl = shaders::get_ctx("webgl")?;
    let vertices = compute_draw_cells_webgl(universe.alive_cells());

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(WebGl::ARRAY_BUFFER, &vert_array, WebGl::STATIC_DRAW);
    }

    context.vertex_attrib_pointer_with_i32(0, 2, WebGl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl::COLOR_BUFFER_BIT);

    for c in (0..vertices.len()).step_by(8) {
        context.draw_arrays(
            WebGl::TRIANGLE_FAN,
            c as i32,
            (8 / 2) as i32,
        );

    }
    Ok(())
}
