mat4 create_projection_matrix(float fov, float aspect) {
    float F = 50.0;
    float N = 0.1;
    float C = 1 / tan(fov/2);

    float X = C / aspect;

    // z' = Az + B
    // z'' = z' / -z
    // So we need an A and B such that z' gets mapped to 0 when z==N and 1 when at z==F
    // (A*N+B) / (-N) = 0 and (A*F+B)/(-F) = 1
    float A = -F/(F-N);
    float B = -(N*F)/(F-N);

    mat4  proj = mat4 ( X,  0,  0,  0,
                        0, -C,  0,  0,
                        0,  0,  A, -1,
                        0,  0,  B,  0);

    return proj;
}


mat4 create_view_matrix(vec3 pos, vec3 dir, vec3 up) {

    /*
     * The premise as follows the component of a vector v onto a basis can be derived as follows
     * cos(t) = c' / |v| because the vector forms the hypotenuse (imagine vector (1, 1) on the cartesian grid)
     * -> c' = |v| * cos(t)
     * -> c' = 1 * |v| * cos(t) so if you are projecting onto a vector 'a' of length 1 then we get
     * -> c' = |a| * |v| * cos(t) which is the dot product
     * -> c' = dot(a,v)
     */

    vec3 f =  normalize(dir);
    vec3 r =  normalize(cross(f, up));
    vec3 u = -normalize(cross(f, r));
    vec3 b = -f;

    vec3 disp = vec3(-dot(r,pos), -dot(u,pos), -dot(b,pos)); // Explain


    mat4 view = mat4(vec4(r.x, u.x, b.x, 0.0),
                     vec4(r.y, u.y, b.y, 0.0),
                     vec4(r.z, u.z, b.z, 0.0),                     
                     vec4(disp, 1.0));

    return view;
}
