#version 330 core

uniform vec4 global_color;

out vec4 fragment_color;  // 최종 출력 색상

void main() {
    fragment_color = global_color;
}