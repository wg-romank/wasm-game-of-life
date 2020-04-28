use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::collections::HashSet;

mod universe;

const CELL_SIZE: u32 = 5;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let canvas = document.get_element_by_id("game-of-life-canvas")?;

    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

pub fn get_ctx() -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let ctx = get_canvas()
        .ok_or(JsValue::from_str("Failed to get canvas"))?
        .get_context("2d")?
        .ok_or(JsValue::from_str("Failed getting ctx"))?;

    ctx.dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|e| JsValue::from(e))
}

pub fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, universe: &universe::Universe) -> Result<(), JsValue> {
    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from_str("gray"));

    let float_width = universe.width() as f64;
    let float_height = universe.height() as f64;
    let csf = CELL_SIZE as f64;

    for i in 0..universe.width() + 1 {
        let fi = i as f64;
        ctx.move_to(fi * (csf + 1.) + 1., 0.);
        ctx.line_to(fi * (csf + 1.) + 1., (csf + 1.) * float_height + 1.);
    }

    for j in 0..universe.height() + 1 {
        let fj = j as f64;
        ctx.move_to(0., fj * (csf + 1.) + 1.);
        ctx.line_to((csf + 1.) * float_width + 1., fj * (csf + 1.) + 1.);
    }

    ctx.stroke();

    Ok(())
}

pub fn draw_cells(
    ctx: &web_sys::CanvasRenderingContext2d,
    universe: &universe::Universe,
    changes: HashSet<(u32,u32)>) -> Result<(), JsValue> {
    let fcs = CELL_SIZE as f64;

    ctx.begin_path();

    for (row, col) in changes {
        let idx = universe.get_index(row, col);

        let stroke_style = match universe.cell(idx) {
            universe::Cell::Dead => "white",
            universe::Cell::Alive => "black"
        };

        ctx.set_fill_style(&JsValue::from(stroke_style));
        ctx.fill_rect(
            (col as f64) * (fcs + 1.) + 1.,
            (row as f64) * (fcs + 1.) + 1.,
            fcs,
            fcs);
    }

    ctx.stroke();

    Ok(())
}

#[wasm_bindgen]
pub fn get_cell_size() -> Result<u32, JsValue> { Ok(CELL_SIZE) }

#[wasm_bindgen]
pub fn setup_canvas(universe: &universe::Universe) -> Result<(), JsValue> {
    let canvas = get_canvas().ok_or(JsValue::from_str("Failed getting canvas"))?;
    canvas.set_width((CELL_SIZE + 1) * universe.width() + 1);
    canvas.set_height((CELL_SIZE + 1) * universe.height() + 1);

    Ok(())
}

#[wasm_bindgen]
pub fn animation_loop(universe: &mut universe::Universe, ticks: u32) -> Result<(), JsValue> {
    // important bit here is that universe needs to be a reference
    // otherwise rust interop will destroy js value
    let ctx = get_ctx()?;

    let changes = universe.tick_many(ticks);

    draw_cells(&ctx, &universe, changes)?;
    draw_grid(&ctx, &universe)?;

    Ok(())
}
