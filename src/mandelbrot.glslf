#version 150 core

out vec4 color;

/* The value in gl_FragCoord is the actual pixel position
 * (e.g. between 0 and 767 since our window has the height of 768).
 * The point of origin (0, 0) is the bottom-left corner (y is going up,
 * x is going right).
 */
void main() {
    vec2 c = gl_FragCoord.xy / 767.0 * 4.0 - 2.0;
    vec2 z = c;
    int i = 0;
    while(i < 10 && length(z) < 2.0) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        i++;
    }

    if(length(z) <= 2.0) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }
}