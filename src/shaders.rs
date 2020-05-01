use web_sys::{WebGlProgram, WebGlRenderingContext as WebGl, WebGlShader};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


fn compile_shader(
    context: &WebGl,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl::COMPILE_STATUS)
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
    context: &WebGl,
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
        .get_program_parameter(&program, WebGl::LINK_STATUS)
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

pub fn setup_shaders() -> Result<WebGl, JsValue> {
    let canvas = get_canvas().ok_or(JsValue::from_str("Failed to get canvas"))?;
    let context: WebGl = get_ctx("webgl")?;

    let vert_shader = compile_shader(
        &context,
        WebGl::VERTEX_SHADER,
        r#"
        precision highp float;

        attribute vec2 position;

        uniform vec2 canvasSize;

        void main() {
            vec2 zeroOne = position / canvasSize;
            vec2 clipSpace = zeroOne * 2.0 - 1.0;
            gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
    "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl::FRAGMENT_SHADER,
        r#"
        precision highp float;
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let canvas_size = context.get_uniform_location(&program, "canvasSize")
        .ok_or(JsValue::from(
            format!("Failed to get uniform uCol: {}", context.get_error())
        )
        )?;

    context.uniform2f(Some(&canvas_size), canvas.width() as f32, canvas.height() as f32);

    Ok(context)
}

pub fn render_pipeline(vertices: &Vec<f32>) -> Result<(), JsValue> {
    let context: WebGl = get_ctx("webgl")?;

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