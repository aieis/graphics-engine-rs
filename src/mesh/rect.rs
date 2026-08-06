pub struct RectMesh {
    pub vertices: [[f32; 2]; 4],
    pub dirty_vertices: bool,

    pub colour: [[f32; 3]; 4],
    pub dirty_colour: bool,

    pub indices: [u16; 6],
    pub dirty_indices: bool,
}

pub const RECT_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

impl RectMesh {
    pub fn new(x: f32, y: f32, width: f32, height: f32, col: [f32; 3]) -> Self {
        let vertices = [
            [x, y],
            [x+width, y],
            [x+width, y+height],
            [x, y+height]
        ];

        let colour = [
            col,
            col,
            col,
            col
        ];

        let indices = RECT_INDICES;

        Self {
            vertices,
            colour,
            indices,
            dirty_vertices: true,
            dirty_colour: true,
            dirty_indices: true,
        }
    }

    pub fn transform(&mut self, rotation: f32, translation: [f32; 2]) {
        let s = rotation.sin();
        let c = rotation.cos();

        for i in 0..self.vertices.len() {
            let x = self.vertices[i][0];
            let y = self.vertices[i][1];

            let xp = x * c - y * s;
            let yp = x * s + y * c;
            self.vertices[i] = [xp + translation[0], yp + translation[1]];
        }

        self.dirty_vertices = true;
    }


    pub fn size_vrt(&self) -> usize {
        std::mem::size_of_val(&self.vertices[..])
    }

    pub fn size_ind(&self) -> usize {
        std::mem::size_of_val(&self.indices[..])
    }

    pub fn size_col(&self) -> usize {
        std::mem::size_of_val(&self.colour[..])
    }
}
