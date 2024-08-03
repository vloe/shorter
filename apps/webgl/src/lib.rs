use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

mod utils;

#[wasm_bindgen]
pub struct WebGLRenderer {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    time_location: WebGlUniformLocation,
    cursor_location: WebGlUniformLocation,
    scroll_location: WebGlUniformLocation,
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
            r#"
attribute vec2 position;
attribute vec3 color;
varying vec3 vColor;
uniform float u_time;
uniform vec2 u_cursor;
uniform float u_scroll;

void main() {
    vec2 pos = position;
    
    float baseFrequency = 6.0;
    float baseAmplitude = 0.06;
    float baseWave = sin(pos.x * baseFrequency + u_time * 0.5) * baseAmplitude;
    
    float scrollEffect = u_scroll * 0.2;
    float scrollWave = sin(pos.x * 3.0 - u_time * 0.3) * scrollEffect;
    
    float distanceToCursor = distance(pos, u_cursor);
    float cursorInfluence = smoothstep(0.4, 0.0, distanceToCursor);
    
    float modifiedFrequency = mix(baseFrequency, baseFrequency * 1.3, cursorInfluence);
    float modifiedAmplitude = mix(baseAmplitude, baseAmplitude * 1.3, cursorInfluence);
    float modifiedPhase = cursorInfluence * sin(u_time * 2.0) * 1.5;
    
    float modifiedWave = sin(pos.x * modifiedFrequency + u_time * 0.5 + modifiedPhase) * modifiedAmplitude;
    
    float finalWave = mix(baseWave, modifiedWave, cursorInfluence) + scrollWave;
    
    pos.y += finalWave;
    gl_Position = vec4(pos, 0.0, 1.0);
    
    vColor = mix(color, vec3(1.0, 0.5, 0.2), cursorInfluence * 0.5);
}
            "#,
        )?;

        let frag_shader = utils::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision mediump float;
            varying vec3 vColor;
            void main() {
                gl_FragColor = vec4(vColor, 1.0);
            }
            "#,
        )?;

        let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let time_location = context.get_uniform_location(&program, "u_time").unwrap();
        let cursor_location = context.get_uniform_location(&program, "u_cursor").unwrap();
        let scroll_location = context.get_uniform_location(&program, "u_scroll").unwrap();

        Ok(WebGLRenderer {
            context,
            program,
            time_location,
            cursor_location,
            scroll_location,
        })
    }

    pub fn render(&self, time: f32, cursor_x: f32, cursor_y: f32, scroll: f32) {
        self.context.clear_color(0.0, 0.0, 0.0, 0.0); // transparent bg
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.uniform1f(Some(&self.time_location), time);
        self.context
            .uniform2f(Some(&self.cursor_location), cursor_x, cursor_y);
        self.context.uniform1f(Some(&self.scroll_location), scroll);

        let num_waves = 200;
        let points_per_wave = 500;
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for i in 0..num_waves {
            let t = i as f32 / num_waves as f32;
            let y_offset = -0.5 + t;
            let amplitude = 0.4 - t * 0.3;
            let phase = t * PI * 2.0;
            let frequency = 1.0 + t * 0.5;

            let color = interpolate_color(t);

            for j in 0..=points_per_wave {
                let x = (j as f32 / points_per_wave as f32) * 2.0 - 1.0;
                let y = amplitude * ((x * frequency * PI * 2.0) + phase).sin() + y_offset;
                vertices.push(x);
                vertices.push(y);

                colors.push(color.0);
                colors.push(color.1);
                colors.push(color.2);
            }
        }

        let vertex_buffer = self.context.create_buffer().unwrap();
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let color_buffer = self.context.create_buffer().unwrap();
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        unsafe {
            let color_array = js_sys::Float32Array::view(&colors);
            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &color_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let position_attribute_location =
            self.context.get_attrib_location(&self.program, "position");
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(position_attribute_location as u32);

        let color_attribute_location = self.context.get_attrib_location(&self.program, "color");
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        self.context.vertex_attrib_pointer_with_i32(
            color_attribute_location as u32,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(color_attribute_location as u32);

        for i in 0..num_waves {
            self.context.draw_arrays(
                WebGlRenderingContext::LINE_STRIP,
                (i * (points_per_wave + 1)) as i32,
                (points_per_wave + 1) as i32,
            );
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.context.viewport(0, 0, width as i32, height as i32);
    }
}

fn interpolate_color(t: f32) -> (f32, f32, f32) {
    let orange = (1.0, 0.5, 0.0);
    let red = (1.0, 0.0, 0.0);

    (
        orange.0 * (1.0 - t) + red.0 * t,
        orange.1 * (1.0 - t) + red.1 * t,
        orange.2 * (1.0 - t) + red.2 * t,
    )
}
