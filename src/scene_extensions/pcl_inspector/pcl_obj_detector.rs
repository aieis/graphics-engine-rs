use crate::geometry::Vec2_Gen;

type Vec2u = Vec2_Gen<usize>;

pub struct PclObj {
    pub seed: Vec2u,
    pub seed_value: f32,

    pub borders: Vec<Vec2u>,
    pub border_values: Vec<f32>,

    pub complete: bool
}


pub struct PclDetector {
    pub objects: Vec<PclObj>
}


impl PclDetector {

    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn seed(&mut self, count: usize, depth_frame_data: &[u16], width: u32, height: u32) {
        let small_dim = count.isqrt();
        let large_dim = if small_dim * small_dim == count { small_dim } else { small_dim + 1};

        let cols = if width < height { small_dim } else { large_dim };
        let rows = count / cols;


        let col_step = width as f32 / cols as f32;
        let row_step = height as f32 / rows as f32;

        let objects = Vec::new();
        objects.reserve(count);

        for i in 0..count {
            let x = ((i % cols) as f32 * col_step) as usize;
            let y = ((i / cols) as f32 * row_step) as usize;

            let v = depth_frame_data[y * width + x] as f32 / 1000.0;

            objects.push_back(PclObj {
                seed: Vec2u { x, y },
            });
        }
    }
}
