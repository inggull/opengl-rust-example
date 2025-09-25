#version 330 core
layout (location = 0) in vec3 position; // vao의 0번 속성으로 자동 할당
layout (location = 1) in vec3 color; // vao의 1번 속성으로 자동 할당

out vec4 vertex_color; // fragment shader로 넘어갈 색상 값

void main() {
    gl_Position = vec4(position, 1.0); // 정점 출력 위치 값을 생성
    vertex_color = vec4(color, 1.0);
}