use gl::attributes::AttributeVector2;
use gl::Ctx;
use gl::Pipeline;
use gl::mesh::Mesh;
use gl::texture::ColorFramebuffer;
use gl::texture::EmptyFramebuffer;
use gl::texture::Framebuffer;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use glsmrs as gl;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn setup_shaders(
    context: &Ctx,
) -> Result<Mesh, JsValue> {
    let (vertices, uvs, indices) = make_quad();

    let mesh = gl::mesh::Mesh::new(&context, &indices)?
        .with_attribute::<AttributeVector2>("position", &vertices)?
        .with_attribute::<AttributeVector2>("uv", &uvs)?;

    Ok(mesh)
}

pub fn shader(context: &Ctx, frag_shader: &str) -> Result<gl::Program, JsValue> {
    gl::Program::new(
        context,
        include_str!("../shaders/dummy.vert"),
        frag_shader,
    )
    .map_err(|e| JsValue::from(e))
}

pub fn setup_display_program(ctx: &Ctx) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/display.frag"))
}

pub fn setup_display_monochrome_program(ctx: &Ctx) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/display_monochrome.frag"))
}

pub fn setup_copy_program(ctx: &Ctx) -> Result<gl::Program, JsValue> {
    shader(ctx, include_str!("../shaders/copy.frag"))
}

pub fn setup_compute_program(context: &Ctx) -> Result<gl::Program, JsValue> {
    gl::Program::new(
        &context,
        include_str!("../shaders/dummy.vert"),
        include_str!("../shaders/compute.frag"),
    )
    .map_err(|e| JsValue::from(e))
}

pub fn make_quad() -> ([[f32; 2]; 4], [[f32; 2]; 4], [u16; 6]) {
    let vertices: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
    let uvs: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    (vertices, uvs, indices)
}

pub fn render_pipeline(
    pipeline: &mut Pipeline,
    display_program: &gl::Program,
    compute_program: &gl::Program,
    copy_program: &gl::Program,
    mesh: &mut Mesh,
    state_fb: &mut ColorFramebuffer,
    display_fb: &mut ColorFramebuffer,
    d_fb: &mut EmptyFramebuffer,
) -> Result<(), JsValue> {
    let uniforms = vec![
        ("canvasSize", gl::UniformData::Vector2(display_fb.viewport().dimensions())),
        (
            "state",
            gl::UniformData::Texture(state_fb.color_slot()),
        ),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    pipeline
        .shade(&compute_program, uniforms, vec![mesh], display_fb)?;

    let copy_uniforms = vec![(
        "state",
        gl::UniformData::Texture(display_fb.color_slot()),
    )]
    .into_iter()
    .collect::<HashMap<&'static str, gl::UniformData>>();

    pipeline
        .shade(&copy_program, copy_uniforms, vec![mesh], state_fb)?;

    let display_uniforms = vec![
        (
            "state",
            gl::UniformData::Texture(state_fb.color_slot()),
        ),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    pipeline
        .shade(&display_program, display_uniforms, vec![mesh], d_fb)?;
    Ok(())
}
