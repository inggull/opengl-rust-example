#version 330 core

uniform vec4 global_color;  // program이 shader에 전달 가능한 전역 변수로, 병렬로 수행되는 모든 shader 스레드들이 동일한 값을 전달받는다

in vec4 vertex_color;  // vertex shader로부터 입련된 변수로, 그것과 같은 변수명과 타입을 사용해야한다

out vec4 fragment_color;  // 최종 출력 색상

void main() {
    fragment_color = vertex_color;
    // fragment_color = global_color;
}