use crate::geometry::Vec2_Gen;

type Vec2u = Vec2_Gen<usize>;

const DEPTH_TOLERANCE: f32 = 0.1;

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

	pub all_ray_objs: Vec<u32>,
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

		self.all_ray_objs.fill(0);

        for i in 0..count {
            let x = ((i % cols) as f32 * col_step) as usize;
            let y = ((i / cols) as f32 * row_step) as usize;

            let idx = y * self.width + x;
            let v = depth_frame_data[idx] as f32 / 1000.0;
            let complete = v == 0.0 || self.all_ray_objs[idx] != 0;

            if !complete {
                self.all_ray_objs[idx] = (i + 1) as u32;
            }

            objects.push(PclObj {
                seed: Vec2u { x, y },
                seed_value: v,
                borders: vec![Vec2u { x, y }],
                border_values: vec![v],
                complete
            });
        }

		self.objects = objects;
    }


	pub fn iterate(&mut self, depth_frame_data: &[u16]) {

		if depth_frame_data.len() != self.all_ray_objs.len() {
			eprintln!("Unexpected data size: Expected {} got {}", self.all_ray_objs.len(), depth_frame_data.len());
			return;
		}

		for i in 0..self.objects.len() {

			if self.objects[i].complete {
				continue;
			}

			let id = (i + 1) as u32;

			let mut new_borders = vec![];
			let mut new_values = vec![];
			let mut same_objs = vec![];

			for j in 0..self.objects[i].borders.len() {
				let p = self.objects[i].borders[j];
				let v = self.objects[i].border_values[j];

				for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
					let x = p.x as isize + dx;
					let y = p.y as isize + dy;

					if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
						continue;
					}

					let n = Vec2u { x: x as usize, y: y as usize };
					let idx = n.y * self.width + n.x;
					let nv = depth_frame_data[idx] as f32 / 1000.0;

					if nv == 0.0 || (nv - v).abs() > DEPTH_TOLERANCE {
						continue;
					}

					match self.all_ray_objs[idx] {
						0 => {
							self.all_ray_objs[idx] = id;
							new_borders.push(n);
							new_values.push(nv);
						}
						other if other != id && !same_objs.contains(&other) => same_objs.push(other),
						_ => {}
					}
				}
			}

			for other in same_objs {
				for o in self.all_ray_objs.iter_mut().filter(|o| **o == other) {
					*o = id;
				}

				let other_obj = &mut self.objects[other as usize - 1];
				other_obj.complete = true;
				new_borders.extend_from_slice(&other_obj.borders);
				new_values.extend_from_slice(&other_obj.border_values);
			}

			let obj = &mut self.objects[i];
			obj.complete = new_borders.is_empty();
			obj.borders = new_borders;
			obj.border_values = new_values;
		}
	}

}
