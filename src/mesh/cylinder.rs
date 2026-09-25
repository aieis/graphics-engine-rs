use crate::geometry::vec3::Vec3;

// TODO: COME BACK TO THIS MODULE


#[allow(unused)]
pub fn make_circle(radius: f32, number_of_segments_per_eighth: usize) -> Vec<Vec3> {
    let n = number_of_segments_per_eighth;

    const {
        assert! (
            !std::mem::needs_drop::<Vec3>(),
            "Type Vec3 needs to be trivially 'removable'. I.e. unsafely forgotten."
        );
    }

    let mut vert = vec![Vec3::new(0.0, 0.0, 0.0); n * 8];
    let da = std::f32::consts::PI / 4.0 / n as f32;

    for i in 0..n {
        let px = radius * (da * i as f32).cos();
        let py = radius * (da * i as f32).sin();
        vert[i]           = Vec3::new( px,  py, 0.0);
        vert[(n-1-i)+n]   = Vec3::new( py,  px, 0.0);
        vert[i+n*2]       = Vec3::new(-py,  px, 0.0);
        vert[(n-1-i)+n*3] = Vec3::new(-px,  py, 0.0);
        vert[i+n*4]       = Vec3::new(-px, -py, 0.0);
        vert[(n-1-i)+n*5] = Vec3::new(-py, -px, 0.0);
        vert[i+n*6]       = Vec3::new( py, -px, 0.0);
        vert[(n-1-i)+n*7] = Vec3::new( px, -py, 0.0);
    }

    vert

}
