mat4 create_projection_matrix(float fov, float aspect) {
    float F = 30.0;
    float N = 0.1;
    float C = 1 / tan(fov);

    float X = C / aspect;

    // z' = Az + B
    // z'' = z' / -z
    // So we need an A and B such that z' gets mapped to 0 when z==N and 1 when at z==F
    // (A*N+B) / (-N) = 0 and (A*F+B)/(-F) = 1
    float A = -F/(F-N);
    float B = -(N*F)/(F-N);

    mat4  proj = mat4 ( X,  0,  0, 0,
                        0,  C,  0, 0,
                        0,  0,  A, B,
                        0,  0, -1, 0);

    return transpose(proj);
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

    vec3 f = normalize(dir);
    vec3 r = normalize(cross(f, up));
    vec3 u = normalize(cross(f, r));

    vec3 disp = vec3(-dot(r,pos), -dot(u,pos), -dot(f,pos)); // Explain


    mat4 view = mat4(vec4(r, 0.0), vec4(u, 0.0), vec4(f, 0.0), vec4(disp, 1.0));

    return view;
}
