use glium::*;
use glium::{
    index::{NoIndices, PrimitiveType},
    uniforms::Uniforms,
};
use glutin::dpi::LogicalSize;
use glutin::*;
use state::*;
use std::env;
use std::f32::consts::PI as PI_32;
use vertex::Vertex;

const FRAGMENT_SHADER: &str = r#"
    #version 140
    #define PI 3.1415926535897932384626433832795
    #define OFFSET 0.4

    in vec2 pos;

    out vec4 f_color;

    void main() {
        vec2 pos = pos * 0.5 * PI;

        f_color = vec4(cos(pos.x) + (1 - pos.x) * OFFSET,
                       sin(pos.x)  + (1 - pos.x) * OFFSET,
                       cos(pos.y) + (1 - pos.y) * OFFSET, {alpha});
    }
"#;

const VERTEX_SHADER: &str = r#"
    #version 140
    #define PI 3.1415926535897932384626433832795

    in vec2 position;
    in vec3 color;

    //out vec3 vColor;
    out vec2 pos;

    void main() {
        float alpha = position.x * 0.5 * PI;
        float beta = position.y * 0.5 * PI;
        vec3 point = vec3(cos(alpha), sin(alpha), cos(beta));

        //vColor = vec3(position, 0.0);
        gl_Position = vec4(position, 0.0, 1.0);
        pos = vec2(position.x, position.y);
    }
"#;

pub fn init_graphics() -> (EventsLoop, Display, Program, DrawParameters<'static>) {
    let events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::from((500, 500)))
        .with_title("rusty-modular-arithmetic")
        .with_resizable(false);
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    let alpha_blend = &env::var("ALPHA_BLENDING")
        .unwrap_or_else(|_| "true".to_string())
        .to_lowercase()
        == "true";

    let (fragment_shader, line_width, blend) = if alpha_blend {
        eprintln!("Alpha-blending enabled");
        (
            FRAGMENT_SHADER.replace("{alpha}", "0.5"),
            Some(2.0),
            Blend::alpha_blending(),
        )
    } else {
        eprintln!("Alpha-blending disabled");
        (
            FRAGMENT_SHADER.replace("{alpha}", "1.0"),
            Some(1.0),
            Blend::default(),
        )
    };

    let shader = Program::from_source(&display, VERTEX_SHADER, &fragment_shader, None).unwrap();

    let params = DrawParameters {
        line_width,
        blend,
        ..Default::default()
    };

    (events_loop, display, shader, params)
}

pub fn draw_frame<U: Uniforms>(
    display: &Display,
    shader: &Program,
    uniforms: &U,
    params: &DrawParameters,
    state: &State,
) {
    let get_cords_from_value = |value: f32| {
        let fraction = value / f32::from(state.divisions);
        let angle = fraction * 2. * PI_32;
        [angle.cos() * state.scale, angle.sin() * state.scale]
    };

    let mut data = Vec::with_capacity(state.divisions as usize * 2);

    for from_value in 0..state.divisions {
        let from_value = f32::from(from_value);
        let from = get_cords_from_value(from_value);

        let to_value = (from_value * state.factor) % f32::from(state.divisions);
        let to = get_cords_from_value(to_value);

        data.push(Vertex::new(from, [1., 1., 1.]));
        data.push(Vertex::new(to, [1., 1., 1.]));
    }

    let vertices = VertexBuffer::new(display, &data).unwrap();
    let indices = NoIndices(PrimitiveType::LinesList);

    let mut new_frame = display.draw();
    new_frame.clear_color(0., 0., 0., 1.);
    new_frame
        .draw(&vertices, &indices, shader, uniforms, params)
        .unwrap();
    new_frame.finish().unwrap();
}
