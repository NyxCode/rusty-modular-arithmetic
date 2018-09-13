use glium::*;
use glium::{
    index::{NoIndices, PrimitiveType},
    uniforms::Uniforms,
};
use glutin::dpi::LogicalSize;
use glutin::*;
use state::*;
use std::f32::consts::PI as PI_32;
use vertex::Vertex;

pub fn init_window() -> (EventsLoop, Display) {
    let events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(LogicalSize::from((700, 700)))
        .with_resizable(false);
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();
    (events_loop, display)
}

pub fn load_line_shader(display: &Display) -> Program {
    Program::from_source(
        display,
        include_str!("line_vertex.glsl"),
        include_str!("line_fragment.glsl"),
        None,
    ).unwrap()
}

pub fn draw_frame<U: Uniforms>(display: &Display, shader: &Program, uniforms: &U, state: &State) {
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
        .draw(&vertices, &indices, shader, uniforms, &Default::default())
        .unwrap();
    new_frame.finish().unwrap();
}
