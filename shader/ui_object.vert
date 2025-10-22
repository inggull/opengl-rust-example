#version 330 core

layout (location = 0) in vec3 position; // vao의 0번 속성으로 자동 할당
layout (location = 1) in vec4 color; // vao의 1번 속성으로 자동 할당
layout (location = 2) in vec2 texture_coord; // vao의 2번 속성으로 자동 할당

uniform mat4 transform;

out vec4 vertex_color; // fragment shader로 넘어갈 색상 값
out vec2 vertex_texture_coord; // fragment shader로 넘어갈 텍스쳐 좌표 값

void main() {
    gl_Position = transform * vec4(position, 1.0); // 정점 출력 위치 값을 생성
    vertex_color = color;
    vertex_texture_coord = texture_coord;
}