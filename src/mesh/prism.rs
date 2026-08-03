use crate::{geometry::vec3::Vec3, utils::colours::{BLUE, CYAN, GREEN, RED, VIOLET, YELLOW}};

use super::mesh::Mesh;


pub fn make_prism(location: Vec3, dimensions: Vec3, col: Vec3) -> Mesh
{
    let lx   = dimensions.x;
    let ly   = dimensions.y;
    let lz   = dimensions.z;

    let x0  = location.x - lx / 2.0;
    let y0  = location.y - ly / 2.0;
    let z0  = location.z - lz / 2.0;

    let vertices = vec![
        // back
        Vec3::new(x0, y0, z0), Vec3::new(x0+lx, y0, z0), Vec3::new(x0+lx, y0+ly, z0), Vec3::new(x0,  y0+ly, z0),

        // front
        Vec3::new(x0, y0, z0+lz), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0,  y0+ly, z0+lz),

        // left
        Vec3::new(x0, y0, z0), Vec3::new(x0, y0, z0+lz), Vec3::new(x0, y0+ly, z0+lz), Vec3::new(x0, y0+ly, z0),

        // right
        Vec3::new(x0+lx, y0, z0), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0),

        // top
        Vec3::new(x0, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0+lz), Vec3::new(x0+lx, y0+ly, z0), Vec3::new(x0, y0+ly, z0),

        // bottom
        Vec3::new(x0, y0, z0+lz), Vec3::new(x0+lx, y0, z0+lz), Vec3::new(x0+lx, y0, z0), Vec3::new(x0, y0, z0),

    ];

    let colour = vec![
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
        col, col, col, col,
    ];


    let indices = vec![
         0,  1,  2,  0,  2,  3,        // back
         4,  6,  5,  4,  7,  6,        // front
         8, 10,  9,  8, 11, 10,        // left
        12, 13, 14, 12, 14, 15,        // right
        16, 18, 17, 16, 19, 18,        // top
        20, 21, 22, 20, 22, 23,        // bottom
    ];

    let center = location;

    let normals = Mesh::create_normals(&vertices, &indices);

    Mesh {
        center,
        vertices,
        colour,
        indices,
        normals,
        dirty_vertices: true,
        dirty_colour: true,
        dirty_indices: true,
        dirty_normals: true,
    }
}


pub fn make_debug_prism(location: Vec3, dimensions: Vec3) -> Mesh {

    let mut mesh = make_prism(location, dimensions, Vec3::of(0.5));

    let colour = vec![
        RED, RED, RED, RED,
        GREEN, GREEN, GREEN, GREEN,
        BLUE, BLUE, BLUE, BLUE,
        CYAN, CYAN, CYAN, CYAN,
        YELLOW, YELLOW, YELLOW, YELLOW,
        VIOLET, VIOLET, VIOLET, VIOLET,
    ];

    mesh.colour = colour;

    return mesh;
}
