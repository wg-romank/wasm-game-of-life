use web_sys::HtmlCanvasElement;
use web_sys::{WebGlRenderingContext as WebGl};
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use glsmrs as gl;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn setup_shaders(canvas: &HtmlCanvasElement, context: &WebGl) -> Result<gl::GlState, JsValue> {
    let (vertices, uvs, indices) = make_quad();

    let mut state = gl::GlState::new(&context, gl::texture::Viewport::new(canvas.width(), canvas.height()));

    let packf32 = |v: &[f32]| { v.iter().flat_map(|el| el.to_ne_bytes().to_vec()).collect::<Vec<u8>>() };
    let packu16 = |v: &[u16]| { v.iter().flat_map(|el| el.to_ne_bytes().to_vec()).collect::<Vec<u8>>() };

    state
        .vertex_buffer("position", packf32(&vertices).as_slice())?
        .vertex_buffer("uv", packf32(&uvs).as_slice())?
        .element_buffer(packu16(&indices).as_slice())?;

    Ok(state)
}

pub fn shader(context: &WebGl, frag_shader: &str) -> Result<gl::Program, JsValue> {
    gl::Program::new(
        &context,
        include_str!("../shaders/dummy.vert"),
        frag_shader,
        vec![
            gl::UniformDescription::new("state", gl::UniformType::Sampler2D),
        ],
        vec![
            gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
            gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
        ]
    ).map_err(|e| JsValue::from(e))
}

pub fn setup_display_program(ctx: &WebGl) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/display.frag"))
}

pub fn setup_display_monochrome_program(ctx: &WebGl) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/display_monochrome.frag"))
}

pub fn setup_copy_program(ctx: &WebGl) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/copy.frag"))
}

pub fn setup_compute_program(context: &WebGl) -> Result<gl::Program, JsValue> {
    gl::Program::new(
        &context,
        include_str!("../shaders/dummy.vert"),
        include_str!("../shaders/compute.frag"),
        vec![
            gl::UniformDescription::new("state", gl::UniformType::Sampler2D),
            gl::UniformDescription::new("canvasSize", gl::UniformType::Vector2),
        ],
        vec![
            gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
            gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
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

pub fn render_pipeline<'a>(
    display_program: &gl::Program,
    compute_program: &gl::Program,
    copy_program: &gl::Program,
    w: u32, h: u32,
    state: &mut gl::GlState,
    state_fb: &mut gl::texture::Framebuffer,
    display_fb: &mut gl::texture::Framebuffer,
) -> Result<(), JsValue> {
    let uniforms = vec![
        ("canvasSize", gl::UniformData::Vector2([w as f32, h as f32]) ),
        ("state", gl::UniformData::Texture(state_fb.color_slot.clone().unwrap())),
    ].into_iter().collect::<HashMap<_, _>>();

    let copy_uniforms = vec![
        ("state", gl::UniformData::Texture(display_fb.color_slot.clone().unwrap())),
    ].into_iter().collect::<HashMap<&'static str, gl::UniformData>>();

    state
        .run_mut(&compute_program, &uniforms, display_fb)?
        .run_mut(&copy_program, &copy_uniforms, state_fb)?
        .run(&display_program, &uniforms)?;
    Ok(())
}
