use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("error compiling shader"))?;
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
            .unwrap_or_else(|| String::from("error compiling shader")))
    }
}

pub fn link_program(
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
            .unwrap_or_else(|| String::from("error linking program")))
    }
}

pub fn interpolate_color(t: f32, color_1: (u8, u8, u8), color_2: (u8, u8, u8)) -> (f32, f32, f32) {
    // convert rgb to float values between 0 and 1
    let color_1_f = (
        color_1.0 as f32 / 255.0,
        color_1.1 as f32 / 255.0,
        color_1.2 as f32 / 255.0,
    );
    let color_2_f = (
        color_2.0 as f32 / 255.0,
        color_2.1 as f32 / 255.0,
        color_2.2 as f32 / 255.0,
    );

    (
        color_1_f.0 * (1.0 - t) + color_2_f.0 * t,
        color_1_f.1 * (1.0 - t) + color_2_f.1 * t,
        color_1_f.2 * (1.0 - t) + color_2_f.2 * t,
    )
}
