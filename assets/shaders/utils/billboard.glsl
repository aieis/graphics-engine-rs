vec3 get_billboard_from_params(vec3 position, vec3 opposition, vec3 camera_position, float offset) {
    if (abs(offset) < 1e-3) {
        return position;
    }

    vec3 up = vec3(0, 0, 1);

    vec3 fv = opposition - position;

    vec3 fv_norm;
    if (length(fv) != 0 ) {
        fv_norm = normalize(fv);
    } else {
        fv_norm = up;
    }

    vec3 view_forward  = normalize(position - camera_position);
    vec3 billboard_vec = normalize(cross(fv_norm, view_forward));
    vec3 billboard_pos = position + billboard_vec * offset;
    return billboard_pos;
}
