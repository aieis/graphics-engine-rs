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
    pub objects: Vec<PclObj>,
	pub height: usize,
	pub width: usize,

	pub all_ray_objs: Vec<usize>,
}


impl PclDetector {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            objects: vec![],
			width,
			height,
			all_ray_objs: vec![0; width * height]
        }
    }

    pub fn seed(&mut self, count: usize, depth_frame_data: &[u16]) {

		if depth_frame_data.len() != self.all_ray_objs.len() {
			eprintln!("Unexpected data size: Expected {} got {}", self.all_ray_objs.len(), depth_frame_data.len());
			return;
		}

        let small_dim = count.isqrt();
        let large_dim = if small_dim * small_dim == count { small_dim } else { small_dim + 1};

        let cols = if self.width < self.height { small_dim } else { large_dim };
        let rows = count / cols;


        let col_step = self.width as f32 / cols as f32;
        let row_step = self.height as f32 / rows as f32;

        let mut objects = Vec::new();
        objects.reserve(count);

        for i in 0..count {
            let x = ((i % cols) as f32 * col_step) as usize;
            let y = ((i / cols) as f32 * row_step) as usize;

            let v = depth_frame_data[y * self.width as usize + x] as f32 / 1000.0;

            objects.push(PclObj {
                seed: Vec2u { x, y },
                seed_value: v,
                borders: vec![],
                border_values: vec![],
                complete: false
            });
        }

		self.objects = objects;
    }


	pub fn iterate(&mut self, depth_frame_data: &[u16]) {

		for obj in self.objects.iter_mut() {

			if depth_frame_data.len() != self.all_ray_objs.len() {
				eprintln!("Unexpected data size: Expected {} got {}", self.all_ray_objs.len(), depth_frame_data.len());
				return;
			}


			// TODO: Check if objs are the same.


		}
	}

}
