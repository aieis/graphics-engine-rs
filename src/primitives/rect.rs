pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub vertices: [[f32; 2]; 4]
}


impl Rect {

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {

        let vertices = [
            [x, y],
            [x+w, y],
            [x+w, y+h],
            [x, y+h]
        ];

        Self {
            x, y, w, h,
            vertices
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

        self.x = self.vertices[0][0];
        self.y = self.vertices[0][1];
        self.w = self.vertices[2][0] - self.x;
        self.h = self.vertices[2][1] - self.h;
    }

    pub const fn size_of_vertices() -> usize {
        std::mem::size_of::<[[f32; 2]; 4]>()
    }
}
