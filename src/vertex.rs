#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Vertex { position, color }
    }
}
