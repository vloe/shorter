use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub(crate) fn compile_shader(
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

pub(crate) fn link_program(
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

pub(crate) fn interpolate_color(t: f32) -> (f32, f32, f32) {
    let purple = (183, 142, 255); // sh purple
    let blue = (60, 114, 239); // sh blue

    // convert rgb to float values between 0 and 1
    let purple_f = (
        purple.0 as f32 / 255.0,
        purple.1 as f32 / 255.0,
        purple.2 as f32 / 255.0,
    );
    let blue_f = (
        blue.0 as f32 / 255.0,
        blue.1 as f32 / 255.0,
        blue.2 as f32 / 255.0,
    );

    (
        purple_f.0 * (1.0 - t) + blue_f.0 * t,
        purple_f.1 * (1.0 - t) + blue_f.1 * t,
        purple_f.2 * (1.0 - t) + blue_f.2 * t,
    )
}
