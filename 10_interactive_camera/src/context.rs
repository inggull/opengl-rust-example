use super::{errors, shader, program, vertex_array, buffer, texture, image};
use nalgebra_glm as glm;

pub struct Context {
    width: u32,
    height: u32,
    program: program::Program,
    vao: vertex_array::VertexArray,
    vbo: buffer::Buffer,
    ebo: buffer::Buffer,
    tbo1: texture::Texture,
    tbo2: texture::Texture,
    mat4: glm::TMat4<f32>,
    cube_positions: Vec<glm::TVec3<f32>>,
    camera_position: glm::TVec3<f32>,
    camera_front: glm::TVec3<f32>,
    up: glm::TVec3<f32>,
    camera_pitch: f32,
    camera_yaw: f32,
    previous_mouse_position: glm::TVec2<f32>,
    mouse_position: glm::TVec2<f32>,
    camera_control: bool,
}

impl Context {
    pub fn create() -> Result<Context, errors::Error> {
        let vertex_shader = shader::Shader::create("shader/camera.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/camera.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());

        let program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());

        let vertices: [f32; 192] = [
            0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
            0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
            -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0,

            -0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
            -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0,

            -0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0,
            -0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0,
            -0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0,

            0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
            0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
            0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0,

            0.5, 0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 1.0,
            0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0,
            -0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0,
            -0.5, 0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 1.0,

            0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0,
            0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0,
            -0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0,
            -0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 1.0
        ];
        let indices: [u32; 36] = [
            0, 1, 2,
            2, 3, 0,

            4, 5, 6,
            6, 7, 4,

            8, 9, 10,
            10, 11, 8,

            12, 13, 14,
            14, 15, 12,

            16, 17, 18,
            18, 19, 16,

            20, 21, 22,
            22, 23, 20,
        ];

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0); // State-setting function
        }

        // 사용할 vao를 먼저 바인딩 해줘야 나머지 오르젝트들이 vao에 저장된다
        let vao = vertex_array::VertexArray::create(); // 새로운 vao를 생성
        vao.bind(); // 사용할 vao를 지정
        let vbo = buffer::Buffer::create(gl::ARRAY_BUFFER, size_of_val(&vertices).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW); // 새로운 vbo를 생성
        let ebo = buffer::Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW); // 새로운 ebo를 생성

        // 초기에 `VertexAttribPointer`의 포인터는 각 버택스 버퍼에 바인딩된 배열의 시작 주소를 가리켰지만, vao가 등장하면서 vao가 가리키는 배열의 오프셋을 의미하게 되었다
        // 속성 0번: position
        vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 8) as i32, (size_of::<f32>() * 0) as *const _); // vao의 0번 속성을 활성화하고, 해당하는 vbo 데이터를 전달
        // 속성 1번: color
        vao.set(1, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 8) as i32, (size_of::<f32>() * 3) as *const _); // vao의 1번 속성을 활성화하고, 해당하는 vbo 데이터를 전달
        // 속성 2번: texture coordinate
        vao.set(2, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 8) as i32, (size_of::<f32>() * 6) as *const _); // vao의 2번 속성을 활성화하고, 해당하는 vbo 데이터를 전달

        let awesomeface = image::Image::load("resources/images/awesomeface.png")?;
        spdlog::info!("Loaded image file \"resources/images/awesomeface.png\" ({} x {}, {} channels)", awesomeface.get_width(), awesomeface.get_height(), awesomeface.get_channel_count());
        let container = image::Image::load("resources/images/container.jpg")?;
        spdlog::info!("Loaded image file \"resources/images/container.jpg\" ({} x {}, {} channels)", container.get_width(), container.get_height(), container.get_channel_count());

        let tbo1 = texture::Texture::create(&awesomeface);
        let tbo2 = texture::Texture::create(&container);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0); // 0번 텍스쳐를 활성화
            tbo1.bind(); // 사용할 tbo를 지정
            gl::ActiveTexture(gl::TEXTURE1); // 1번 텍스쳐를 활성화
            tbo2.bind(); // 사용할 tbo를 지정

            program.use_();  // 사용할 프로그램을 지정
            program.set_uniform1i("texture0\0", 0); // 프로그램의 전역 변수 `texture0`에 0을 할당
            program.set_uniform1i("texture1\0", 1); // 프로그램의 전역 변수 `texture1`에 1을 할당

            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::Enable(gl::BLEND);
            gl::Enable(gl::DEPTH_TEST);
        }

        // 4x4 단위 행렬
        let mat4 = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let cube_positions = vec![
            glm::vec3::<f32>(-2.0, 0.0, -2.0),
            glm::vec3::<f32>(0.0, 0.0, -2.0),
            glm::vec3::<f32>(2.0, 0.0, -2.0),
            glm::vec3::<f32>(-2.0, 0.0, 0.0),
            glm::vec3::<f32>(0.0, 0.0, 0.0),
            glm::vec3::<f32>(2.0, 0.0, 0.0),
            glm::vec3::<f32>(-2.0, 0.0, 2.0),
            glm::vec3::<f32>(0.0,  0.0, 2.0),
            glm::vec3::<f32>(2.0, 0.0, 2.0),
        ];

        let width = super::WINDOW_WIDTH;
        let height = super::WINDOW_HEIGHT;

        // let vec4 = glm::vec4(1.0, 0.0, 0.0, 1.0); // 위치 (1, 0, 0)의 `점` 동차 좌표
        let model = glm::rotate(&mat4, glm::pi::<f32>() / 180.0 * 30.0, &glm::vec3(1.0, 0.0, 0.0)); // 단위 행렬 기준 x축으로 30도만큼 회전하는 행렬
        let view = glm::translate(&mat4, &glm::vec3(0.0, 0.0, -3.0)); // 단위 행렬 기준 (0.0, 0.0, -3)만큼 평행 이동 하는 행렬
        let projection = glm::perspective(width as f32 / height as f32, glm::pi::<f32>() / 180.0 * 45.0, 0.01, 10.0); // 종횡비 16:9, 세로 화각 45도의 원근 투영
        let scale = glm::scale(&mat4, &glm::vec3(0.5, 0.5, 0.5)); // 단위 행렬 기준 모든 축에 대해 0.5배 확대하는 행렬
        // let result = translate * rotate * scale * vec4; // 확대, 회전, 평행 이동 순으로 점에 선형 변환 적용
        // spdlog::info!("Transformated vec4: [{}, {}, {}]", result.x, result.y, result.z);
        let transform = mat4 * projection * view * model * scale;
        program.set_uniform_matrix4fv("transform\0", &transform); // 프로그램의 전역 변수 `transform`에 4차원 형렬의 주소값을 할당

        // 카메라 위치 정보
        let camera_position = glm::vec3(0.0, 0.0, 3.0); // 카메라 위치
        let camera_front = glm::vec3(0.0, 0.0, -1.0); // 카메라가 바라보는 방향
        let up = glm::vec3(0.0, 1.0, 0.0); // 화면 상의 위쪽 방향

        let camera_pitch = 0f32;
        let camera_yaw = 0f32;

        // 마우스 위치 정보
        let previous_mouse_position = glm::vec2(0.0, 0.0);
        let mouse_position = glm::vec2(0.0, 0.0);

        let camera_control = false;

        Ok(Context { width, height, program, vao, vbo, ebo, tbo1, tbo2, mat4, cube_positions, camera_position, camera_front, up, camera_pitch, camera_yaw, previous_mouse_position, mouse_position, camera_control })
    }

    pub fn render(&mut self, time: f32) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // State-using function
            self.program.use_(); // 사용할 프로그램을 지정
            let view = glm::look_at(&self.camera_position, &(&self.camera_position + &self.camera_front), &self.up);
            let projection = glm::perspective(self.width as f32 / self.height as f32, glm::pi::<f32>() / 180.0 * 45.0, 0.01, 20.0);

            for (index, cube_position) in self.cube_positions.iter().enumerate() {
                let position = cube_position;
                let mut model = glm::translate(&self.mat4, position);
                model = glm::rotate(&model, glm::pi::<f32>() * time * 0.5 + 10.0 * index as f32, &glm::vec3(1.0, 0.3 , 0.5));
                let transform = self.mat4 * projection * view * model;
                self.program.set_uniform_matrix4fv("transform\0", &transform);
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, std::ptr::null());
            }
        }
    }

    pub fn process_input(&mut self, window: &glfw::PWindow, delta_time: f32) {
        let camera_speed = delta_time * 2.5;
        // let camera_direction = glm::normalize(&(self.camera_position - self.camera_target));
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.camera_position += camera_speed * &self.camera_front;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.camera_position -= camera_speed * &self.camera_front;
        }

        let camera_right = glm::normalize(&glm::cross(&self.up, &-self.camera_front));
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.camera_position += camera_speed * &camera_right;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.camera_position -= camera_speed * &camera_right;
        }

        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            self.camera_position += camera_speed * &self.up;
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            self.camera_position -= camera_speed * &self.up;
        }
    }

    pub fn reshape(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_position = glm::vec2(x as f32, y as f32);

        if self.camera_control == false {
            return;
        }

        let delta_position = self.mouse_position - self.previous_mouse_position;

        let sensitivity: f32 = 0.1;
        self.camera_yaw -= delta_position.x * sensitivity;
        self.camera_pitch -= delta_position.y * sensitivity;
        if self.camera_yaw < 0.0 {
            self.camera_yaw += 360.0;
        }
        if self.camera_yaw > 360.0 {
            self.camera_yaw -= 360.0;
        }
        if self.camera_pitch < -89.0 {
            self.camera_pitch = -89.0;
        }
        if self.camera_pitch > 89.0 {
            self.camera_pitch = 89.0;
        }

        self.camera_front = (glm::rotate(&self.mat4, glm::pi::<f32>() / 180.0 * self.camera_yaw, &glm::vec3(0.0, 1.0, 0.0)) * glm::rotate(&self.mat4, glm::pi::<f32>() / 180.0 * self.camera_pitch, &glm::vec3(1.0, 0.0, 0.0)) * glm::vec4(0.0, 0.0, -1.0, 0.0)).xyz();

        self.previous_mouse_position = self.mouse_position;
    }

    pub fn mouse_button(&mut self, mouse_button: glfw::MouseButton, action: glfw::Action) {
        if mouse_button == glfw::MouseButtonRight {
            match action {
                glfw::Action::Release => self.camera_control = false,
                _ => {
                    self.previous_mouse_position = self.mouse_position;
                    self.camera_control = true;
                }
            }
        }
    }
}