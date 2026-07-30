use crate::geometry::vec3::Vec3;

pub struct Mesh {
    pub center: Vec3,

    pub vertices: Vec<Vec3>,
    pub dirty_vertices: bool,

    pub colour: Vec<Vec3>,
    pub dirty_colour: bool,

    pub normals: Vec<Vec3>,
    pub dirty_normals: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,
}


impl Mesh {

    pub fn translate(&mut self, t: Vec3) {
        self.center += t;
        for i in 0..self.vertices.len() {
            self.vertices[i] += t;
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_z(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x * c - y * s,
                y: cn.y + x * s + y * c,
                z: cn.z + z
            };
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_y(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x * c - z * s,
                y: cn.y + y,
                z: cn.z + x * s + z * c,
            };
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_x(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x,
                y: cn.y + z * s + y * c,
                z: cn.z + z * c - y * s,
            };
        }

        self.dirty_vertices = true;
    }

    pub fn recompute_normals(&mut self) {

        Self::update_normals(&self.vertices, &self.indices, &mut self.normals);
        self.dirty_normals = true;

    }

    pub fn set_colour(&mut self, colour: [f32; 3]) {
        self.colour.fill(Vec3::from_slice(colour));
        self.dirty_colour = true;
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

    pub fn size_normals(&self) -> usize {
        std::mem::size_of_val(&self.normals[..])
    }

    pub fn create_normals(vertices: &Vec<Vec3>, indices: &Vec<u16>) -> Vec<Vec3> {

        let mut normals = vec![];
        normals.resize(vertices.len(), Vec3::X);

        Self::update_normals(vertices, indices, &mut normals);

        return normals;

    }


    pub fn update_normals(vertices: &Vec<Vec3>, indices: &Vec<u16>, normals: &mut Vec<Vec3>) {

        let num_verts: usize = vertices.len();
        let num_tris : usize = indices.len() / 3;

        normals.resize(num_verts, Vec3::X);

        for t in 0..num_tris {
            let i0 = indices[t * 3 + 0];
            let i1 = indices[t * 3 + 1];
            let i2 = indices[t * 3 + 2];

            let v0 = vertices[i0 as usize];
            let v1 = vertices[i1 as usize];
            let v2 = vertices[i2 as usize];

            let n  = Vec3::cross(v1 - v0, v2 - v0);

            normals[i0 as usize] += n;
            normals[i1 as usize] += n;
            normals[i2 as usize] += n;
        }

        for i in 0..num_verts {
            normals[i] = Vec3::norm(&normals[i]);
        }
    }


}
