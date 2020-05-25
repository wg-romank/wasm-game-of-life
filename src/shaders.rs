use web_sys::{WebGlRenderingContext as WebGl};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::collections::HashMap;

use glsmrs as gl;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let canvas = document.get_element_by_id("game-of-life-canvas")?;

    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

fn get_ctx<T : JsCast>(ctx_name: &str) -> Result<T, JsValue> {
    let ctx = get_canvas()
        .ok_or(JsValue::from_str("Failed to get canvas"))?
        .get_context(ctx_name)?
        .ok_or(JsValue::from_str("Failed getting ctx"))?;

    ctx.dyn_into::<T>()
        .map_err(|e| JsValue::from(e))
}

pub fn setup_shaders() -> Result<gl::GlState, JsValue> {
    let canvas = get_canvas().ok_or(JsValue::from_str("Failed to get canvas"))?;
    let context: WebGl = get_ctx("webgl")?;

    // todo compile program here
    let state = gl::GlState::new(&context, gl::Viewport {w: canvas.width(), h: canvas.height()});

    Ok(state)
}

pub fn render_pipeline(
    state: &mut gl::GlState,
    vertices: &[f32],
    elements: &[u16]
) -> Result<(), JsValue> {
    let context: WebGl = get_ctx("webgl")?;

    // todo: figure out how to pass it around instead of compiling all the time
    let program = gl::Program::new(
        &context,
        include_str!("../shaders/display.vert"),
        include_str!("../shaders/display.frag"),
        vec![
            gl::UniformDescription::new("canvasSize", gl::UniformType::Vector2)
        ],
        vec![
            gl::AttributeDescription::new("position", gl::AttributeType::Vector2)
        ]
    )?;


    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl::COLOR_BUFFER_BIT);

    let uniforms: HashMap<_, _> = vec![
        // todo: use actual size instead of hardcoded
        ("canvasSize", gl::UniformData::Vector2([395.0, 395.0]) )
    ].into_iter().collect();

    let vb: Vec<u8> = vertices.iter().flat_map(|v| v.to_ne_bytes().to_vec()).collect();
    let eb: Vec<u8> = elements.iter().flat_map(|e| e.to_ne_bytes().to_vec()).collect();

    state
        .vertex_buffer("position", vb.as_slice())?
        .element_buffer(eb.as_slice())?
        .run(&program, &uniforms)?;

    Ok(())
}