#version 330 core

uniform vec3 global_position;  // 프로그램이 셰이더에 전달 가능한 전역 변수로, 병렬로 수행되는 모든 셰이더 스레드들이 동일한 값을 전달받는다

void main() {
    gl_Position = vec4(global_position, 1.0);  // 정점 출력 위치 값을 생성
}

// 기본 타입: int, float, double, uint, bool
// N: 2 ~ 4,
// 벡터 타입: ivecN, vecN, dvecN, uvecN, bvecN
// 행렬 타입: imetN, metN, dmetN, umetN, bmetN