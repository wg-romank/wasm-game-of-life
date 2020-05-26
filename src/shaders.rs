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

    let (vertices, uvs, indices) = make_quad();

    let mut state = gl::GlState::new(&context, gl::Viewport {w: canvas.width(), h: canvas.height()});

    let packf32 = |v: &[f32]| { v.iter().flat_map(|el| el.to_ne_bytes().to_vec()).collect::<Vec<u8>>() };
    let packfu16 = |v: &[u16]| { v.iter().flat_map(|el| el.to_ne_bytes().to_vec()).collect::<Vec<u8>>() };

    state
        .vertex_buffer("position", packf32(&vertices).as_slice())?
        .vertex_buffer("uv", packf32(&uvs).as_slice())?
        .element_buffer(packfu16(&indices).as_slice())?
        .texture("state", None, canvas.width(), canvas.height())?
        .texture("display", None, canvas.width(), canvas.height())?;

    Ok(state)
}

pub fn setup_init_program() -> Result<gl::Program, JsValue> {
    let context: WebGl = get_ctx("webgl")?;

    gl::Program::new(
        &context,
        include_str!("../shaders/dummy.vert"),
        include_str!("../shaders/init.frag"),
        vec![
            gl::UniformDescription::new("canvasSize", gl::UniformType::Vector2)
        ],
        vec![
            gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
            gl::AttributeDescription::new("uv", gl::AttributeType::Vector2)
        ]
    ).map_err(|e| JsValue::from(e))
}

pub fn setup_program() -> Result<gl::Program, JsValue> {
    let context: WebGl = get_ctx("webgl")?;

    gl::Program::new(
        &context,
        include_str!("../shaders/display.vert"),
        include_str!("../shaders/display.frag"),
        vec![
            gl::UniformDescription::new("canvasSize", gl::UniformType::Vector2)
        ],
        vec![
            gl::AttributeDescription::new("position", gl::AttributeType::Vector2)
        ]
    ).map_err(|e| JsValue::from(e))
}

pub fn make_quad() -> ([f32; 8], [f32; 8], [u16; 6]) {
    let vertices: [f32; 8] = [
        -1.0, -1.0,
        1.0, -1.0,
        1.0, 1.0,
        -1.0, 1.0
    ];
    let uvs: [f32; 8] = [
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0,
        0.0, 1.0
    ];
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    (vertices, uvs, indices)
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn render_pipeline(
    program: &gl::Program,
    state: &mut gl::GlState,
    vertices: &[f32],
    elements: &[u16]
) -> Result<(), JsValue> {
    let canvas = get_canvas().ok_or(JsValue::from_str("Failed to get canvas"))?;
    let context: WebGl = get_ctx("webgl")?;

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl::COLOR_BUFFER_BIT);

    let (w, h) = (32, 32);
    // let (w, h) = (canvas.width(), canvas.height());
    log!("Canvas {} {}", w, h);

    let uniforms: HashMap<_, _> = vec![
        // todo: use actual size instead of hardcoded
        ("canvasSize", gl::UniformData::Vector2([w as f32, h as f32]) )
    ].into_iter().collect();

    // let vb: Vec<u8> = vertices.iter().flat_map(|v| v.to_ne_bytes().to_vec()).collect();
    // let eb: Vec<u8> = elements.iter().flat_map(|e| e.to_ne_bytes().to_vec()).collect();

    state
        // .vertex_buffer("position", vb.as_slice())?
        // .element_buffer(eb.as_slice())?
        .run(&program, &uniforms)?;

    Ok(())
}