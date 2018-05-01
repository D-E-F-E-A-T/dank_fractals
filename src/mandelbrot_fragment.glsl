#version 140

const int ITERATIONS = 100;

uniform vec2 cam_position;
uniform float cam_zoom;
uniform float screen_height;

out vec4 color;

vec2 map_frag_to_complex_plane(vec2 fragCoord) {
    return (fragCoord / screen_height * 4.0 - 2.0) / cam_zoom - cam_position;
}

void main() {
    vec2 c = map_frag_to_complex_plane(gl_FragCoord.xy);

    vec2 z = c;
    int iteration;
    for (iteration = 0;iteration < ITERATIONS; iteration++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
    }

    if(length(z) <= 2.0) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}