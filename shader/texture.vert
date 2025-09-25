#version 330 core

layout (location = 0) in vec3 position; // vao의 0번 속성으로 자동 할당
layout (location = 1) in vec3 color; // vao의 1번 속성으로 자동 할당
layout (location = 2) in vec2 texture_coord; // vao의 2번 속성으로 자동 할당

out vec4 vertex_color; // fragment shader로 넘어갈 색상 값
out vec2 vertex_texture_coord; // fragment shader로 넘어갈 텍스쳐 좌표 값

void main() {
    gl_Position = vec4(position, 1.0); // 반드시 정점의 출력 위치 값을 계산해야 한다
    vertex_color = vec4(color, 1.0);
    vertex_texture_coord = texture_coord;
}