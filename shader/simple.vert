#version 330 core
layout (location = 0) in vec3 position;  // attribute #0
layout (location = 1) in vec3 color;  // attribute #1

out vec4 vertex_color;  // fragment shader로 넘어갈 색상 값

void main() {
    gl_Position = vec4(position, 1.0);  // 정점 출력 위치 값을 생성
    vertex_color = vec4(color, 1.0);
}

// 기본 타입: int, float, double, uint, bool
// N: 2 ~ 4,
// 벡터 타입: ivecN, vecN, dvecN, uvecN, bvecN
// 행렬 타입: imetN, metN, dmetN, umetN, bmetN