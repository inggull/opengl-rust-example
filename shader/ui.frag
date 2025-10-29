#version 330 core

uniform sampler2D texture0;
uniform int shader_type;

in vec4 vertex_color; // vertex shader로부터 입련된 변수로, 그것과 같은 변수명과 타입을 사용해야 한다
in vec2 vertex_texture_coord; // vertex shader로부터 입력된 변수로, 같은 변수명에 같은 타입을 사용해야 한다

out vec4 fragment_color; // 최종 출력 색상

void main() {
    vec4 color0;
    switch (shader_type) {
        case 0: // color
            fragment_color = vertex_color;
            break;
        case 1: // texture
            color0 = texture(texture0, vertex_texture_coord);
            fragment_color = color0;
            break;
        case 2: // mix
            color0 = texture(texture0, vertex_texture_coord);
            fragment_color = vec4(vertex_color.r, vertex_color.g, vertex_color.b, color0.a);
            break;
    }
}