use std::ops::Not;

use gl::Pipeline;
use gl::texture::ColorFormat;
use wasm_bindgen::prelude::*;

use glsmrs as gl;
use gl::Ctx;
use gl::GL;
use gl::texture::TextureSpec;
use gl::{texture::{Framebuffer, Viewport, UploadedTexture}, mesh::Mesh};

mod shaders;

#[wasm_bindgen]
pub struct Render {
    ctx: Ctx,
    mesh: Mesh,
    display_program: gl::Program,
    monochrome_display_program: gl::Program,
    compute_program: gl::Program,
    copy_program: gl::Program,
    state_fb: Framebuffer<Rc<UploadedTexture>, ()>,
    display_fb: Framebuffer<Rc<UploadedTexture>, ()>,
    pipeline: Pipeline,
    color: bool,
}

use std::rc::Rc;

#[wasm_bindgen]
impl Render {
    pub fn new(w: u32, h: u32) -> Result<Render, JsValue> {
        let canvas =
            gl::util::get_canvas("game-of-life-canvas").ok_or(format!("failed to find canvas"))?;
        let ctx = Ctx::new(gl::util::get_ctx_from_canvas(&canvas, "webgl")?)?;

        let mesh = shaders::setup_shaders(&ctx)?;
        let display_program = shaders::setup_display_program(&ctx)?;
        let monochrome_display_program = shaders::setup_display_monochrome_program(&ctx)?;
        let compute_program = shaders::setup_compute_program(&ctx)?;

        let texture_spec = TextureSpec::pixel(ColorFormat(GL::RGBA), [w, h]);

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

        let state_texture = Rc::new(texture_spec.upload_u8(&ctx, &tex_state)?);
        let display_texture = Rc::new(texture_spec.upload(&ctx, None)?);

        let vpp = Viewport::new(w, h);

        let state_fb =
            Framebuffer::new(&ctx, vpp)?.with_color_slot(&ctx, state_texture);
        let display_fb =
            Framebuffer::new(&ctx, vpp)?.with_color_slot(&ctx, display_texture);

        let pipeline = Pipeline::new(Viewport::new(canvas.width(), canvas.height()));

        let copy_program = shaders::setup_copy_program(&ctx)?;

        Ok(Self {
            ctx,
            mesh,
            display_program,
            monochrome_display_program,
            compute_program,
            copy_program,
            state_fb,
            display_fb,
            pipeline,
            color: true,
        })
    }

    pub fn toggle_color(&mut self) {
        self.color = self.color.not();
    }

    pub fn frame(&mut self) -> Result<(), JsValue> {
        shaders::render_pipeline(
            &self.pipeline,
            &self.ctx,
            if self.color {
                &self.display_program
            } else {
                &self.monochrome_display_program
            },
            &self.compute_program,
            &self.copy_program,
            &mut self.mesh,
            &mut self.state_fb,
            &mut self.display_fb,
        )
    }
}
