use std::ops::Not;

use gl::texture::{Framebuffer, Viewport};
use wasm_bindgen::prelude::*;

use web_sys::WebGlRenderingContext as WebGl;

use glsmrs as gl;

mod shaders;

#[wasm_bindgen]
pub struct Render {
    state: gl::GlState,
    display_program: gl::Program,
    monochrome_display_program: gl::Program,
    compute_program: gl::Program,
    copy_program: gl::Program,
    w: u32,
    h: u32,
    state_fb: gl::texture::Framebuffer,
    display_fb: gl::texture::Framebuffer,
    color: bool,
}

#[wasm_bindgen]
impl Render {
    pub fn new(w: u32, h: u32) -> Result<Render, JsValue> {
        let canvas =
            gl::util::get_canvas("game-of-life-canvas").ok_or(format!("failed to find canvas"))?;
        let ctx: WebGl = gl::util::get_ctx_from_canvas(&canvas, "webgl")?;

        let state = shaders::setup_shaders(&canvas, &ctx)?;
        let display_program = shaders::setup_display_program(&ctx)?;
        let monochrome_display_program = shaders::setup_display_monochrome_program(&ctx)?;
        let compute_program = shaders::setup_compute_program(&ctx)?;

        let texture_spec = gl::texture::TextureSpec {
            color_format: WebGl::RGBA,
            dimensions: [w, h],
            interpolation_min: WebGl::NEAREST,
            interpolation_mag: WebGl::NEAREST,
            wrap_t: WebGl::CLAMP_TO_EDGE,
            wrap_s: WebGl::CLAMP_TO_EDGE,
        };

        let tex_state = (0..w * h)
            .map(|idx: u32| {
                if js_sys::Math::random() > 0.9 || idx % 7 == 0 {
                    1
                } else {
                    0
                }
            })
            .flat_map(|v: u32| v.to_ne_bytes().to_vec())
            .collect::<Vec<u8>>();

        let state_texture = texture_spec.upload(&ctx, Some(&tex_state))?;
        let display_texture = texture_spec.upload(&ctx, None)?;

        let state_fb =
            Framebuffer::new(&ctx, Viewport::new(w, h))?.with_color_slot(&ctx, state_texture);
        let display_fb =
            Framebuffer::new(&ctx, Viewport::new(w, h))?.with_color_slot(&ctx, display_texture);

        let copy_program = shaders::setup_copy_program(&ctx)?;

        Ok(Self {
            state,
            display_program,
            monochrome_display_program,
            compute_program,
            copy_program,
            w,
            h,
            state_fb,
            display_fb,
            color: true,
        })
    }

    pub fn toggle_color(&mut self) {
        self.color = self.color.not();
    }

    pub fn frame(&mut self) -> Result<(), JsValue> {
        shaders::render_pipeline(
            if self.color {
                &self.display_program
            } else {
                &self.monochrome_display_program
            },
            &self.compute_program,
            &self.copy_program,
            self.w,
            self.h,
            &mut self.state,
            &mut self.state_fb,
            &mut self.display_fb,
        )
    }
}
