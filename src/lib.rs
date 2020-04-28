use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::collections::HashSet;

mod universe;

const CELL_SIZE: u32 = 5;

fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
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

fn draw_grid(ctx: &web_sys::CanvasRenderingContext2d, universe: &universe::Universe) -> Result<(), JsValue> {
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

fn draw_cells(
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
    let ctx = get_ctx("2d")?;

    let changes = universe.tick_many(ticks);

    draw_cells(&ctx, &universe, changes)?;
    draw_grid(&ctx, &universe)?;

    Ok(())
}

fn setup_shaders() -> Result<web_sys::WebGlRenderingContext, JsValue> {
    let context = get_ctx("webgl")?;

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision highp float;
        uniform float uCol;
        attribute vec4 position;
        void main() {
            gl_Position = position;
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision highp float;
        uniform float uCol;
        void main() {
            gl_FragColor = vec4(uCol * 1.0, 1.0, 1.0, 1.0);
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let u_color = context.get_uniform_location(&program, "uCol")
        .ok_or(JsValue::from(
            format!("Failed to get uniform uCol: {}", context.get_error())
        )
        )?;

    context.uniform1f(Some(&u_color), 0.5);

    Ok(context)
}

#[wasm_bindgen]
pub fn animation_webgl() -> Result<(), JsValue> {
    let context = setup_shaders()?;

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}