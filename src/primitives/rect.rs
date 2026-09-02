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

	pub fn refresh_vertices(&mut self) {
		self.vertices = [
            [self.x,        self.y],
            [self.x+self.w, self.y],
            [self.x+self.w, self.y+self.h],
            [self.x,        self.y+self.h]
        ];
	}

    pub const fn size_of_vertices() -> usize {
        std::mem::size_of::<[[f32; 2]; 4]>()
    }
}
