#version 330 core

uniform sampler2D texture0;
uniform sampler2D texture1;

in vec2 vertex_texture_coord; // vertex shader로부터 입력된 변수로, 같은 변수명에 같은 타입을 사용해야 한다

out vec4 fragment_color; // 최종 출력 색상

void main() {
    vec4 color0 = texture(texture0, vertex_texture_coord);
    vec4 color1 = texture(texture1, vertex_texture_coord);
    fragment_color = mix(color0, color1, 0.8);
}