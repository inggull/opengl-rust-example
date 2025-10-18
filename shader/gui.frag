#version 330 core

in vec4 vertex_color; // vertex shader로부터 입련된 변수로, 그것과 같은 변수명과 타입을 사용해야 한다

out vec4 fragment_color; // 최종 출력 색상

void main() {
    fragment_color = vertex_color;
}