use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};

mod utils;

const NUM_WAVES: usize = 170;
const POINTS_PER_WAVE: usize = 200;
const COLOR_1: (u8, u8, u8) = (82, 58, 137); // slightly darker purple
const COLOR_2: (u8, u8, u8) = (54, 102, 215); // slightly darker blue

#[wasm_bindgen]
pub struct WebGLRenderer {
    context: WebGlRenderingContext,
    program: WebGlProgram,
}

#[wasm_bindgen]
impl WebGLRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<WebGLRenderer, JsValue> {
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let vert_shader = utils::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("../shaders/vertex.glsl"),
        )?;

        let frag_shader = utils::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("../shaders/fragment.glsl"),
        )?;

        let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let renderer = WebGLRenderer { context, program };

        renderer.setup_buffers()?;

        Ok(renderer)
    }

    fn setup_buffers(&self) -> Result<(), JsValue> {
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for i in 0..NUM_WAVES {
            let t = i as f32 / NUM_WAVES as f32;
            for j in 0..=POINTS_PER_WAVE {
                let x = (j as f32 / POINTS_PER_WAVE as f32) * 2.0 - 1.0;
                vertices.extend_from_slice(&[x, t]);

                let color = utils::interpolate_color(t, COLOR_1, COLOR_2);
                colors.extend_from_slice(&[color.0, color.1, color.2]);
            }
        }

        self.buffer_data(WebGlRenderingContext::ARRAY_BUFFER, &vertices, "vertex");
        self.buffer_data(WebGlRenderingContext::ARRAY_BUFFER, &colors, "color");

        Ok(())
    }

    fn buffer_data(&self, target: u32, data: &[f32], buffer_name: &str) {
        let buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(target, Some(&buffer));
        unsafe {
            let array = js_sys::Float32Array::view(data);
            self.context.buffer_data_with_array_buffer_view(
                target,
                &array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }
        js_sys::Reflect::set(&self.context, &JsValue::from_str(buffer_name), &buffer).unwrap();
    }

    #[wasm_bindgen]
    pub fn render(&self, time: f32, cursor_x: f32, cursor_y: f32) {
        self.context.clear_color(0.0, 0.0, 0.0, 0.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        if let Some(loc) = self.context.get_uniform_location(&self.program, "u_time") {
            self.context.uniform1f(Some(&loc), time);
        }
        if let Some(loc) = self.context.get_uniform_location(&self.program, "u_cursor") {
            self.context.uniform2f(Some(&loc), cursor_x, cursor_y);
        }

        let position_loc = self.context.get_attrib_location(&self.program, "position") as u32;
        let color_loc = self.context.get_attrib_location(&self.program, "color") as u32;

        let vertex_buffer =
            js_sys::Reflect::get(&self.context, &JsValue::from_str("vertex")).unwrap();
        self.context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&vertex_buffer.dyn_into::<web_sys::WebGlBuffer>().unwrap()),
        );
        self.context.vertex_attrib_pointer_with_i32(
            position_loc,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(position_loc);

        let color_buffer =
            js_sys::Reflect::get(&self.context, &JsValue::from_str("color")).unwrap();
        self.context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&color_buffer.dyn_into::<web_sys::WebGlBuffer>().unwrap()),
        );
        self.context.vertex_attrib_pointer_with_i32(
            color_loc,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context.enable_vertex_attrib_array(color_loc);

        for i in 0..NUM_WAVES {
            self.context.draw_arrays(
                WebGlRenderingContext::LINE_STRIP,
                (i * (POINTS_PER_WAVE + 1)) as i32,
                (POINTS_PER_WAVE + 1) as i32,
            );
        }
    }

    #[wasm_bindgen]
    pub fn resize(&self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
    }
}
