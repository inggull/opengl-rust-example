use super::{errors, shader, program};
use glad::gl;

pub struct Context {
    program: program::Program,
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Context {
    pub fn create() -> Result<Context, errors::Error> {
        let vertex_shader = shader::Shader::create("shader/triangle.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/triangle.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());

        let program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());

        let vertices: [f32; 18] = [
            0.0, 0.5, 0.0, 1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.5, -0.5, 0.0, 0.0, 0.0, 1.0,
        ];
        let indices: [u32; 3] = [
            0, 1, 2,
        ];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        // 사용할 vao를 먼저 바인딩 해줘야 나머지 오르젝트들이 vao에 저장된다
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);  // State-setting function
            gl::GenVertexArrays(1, &mut vao);  // 새로운 vao를 생성
            spdlog::info!("Created vertex array({})", vao);
            gl::BindVertexArray(vao);  // 사용할 vao를 지정

            gl::GenBuffers(1, &mut vbo);  // 새로운 vbo를 생성
            spdlog::info!("Created vertex buffer({})", vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  // 사용할 vbo를 지정
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertices).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW);  // vbo에 데이터를 전달

            // 초기에 `VertexAttribPointer`의 포인터는 각 버택스 버퍼에 바인딩된 배열의 시작 주소를 가리켰지만, vao가 등장하면서 vao가 가리키는 배열의 오프셋을 의미하게 되었다
            // 속성 0번: position
            gl::EnableVertexAttribArray(0);  // vao의 속성 0번을 활성화
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 6) as i32, (size_of::<f32>() * 0) as *const _);  // vao의 속성 0번에 해당하는 vbo 데이터를 전달

            // 속성 1번: color
            gl::EnableVertexAttribArray(1);  // vao의 속성 1번을 활성화
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 6) as i32, (size_of::<f32>() * 3) as *const _);  // vao의 속성 1번에 해당하는 vbo 데이터를 전달

            gl::GenBuffers(1, &mut ebo);  // 새로운 ebo를 생성
            spdlog::info!("Created element buffer({})", ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);  // 사용할 ebo를 지정
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW);  // ebo에 데이터를 전달
        }

        Ok(Context { program, vao, vbo, ebo })
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);  // State-using function
            gl::UseProgram(self.program.get());  // 사용할 프로그램을 지정
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
        spdlog::info!("Dropped vertex array({})", self.vao);
        spdlog::info!("Dropped vertex buffer({})", self.vbo);
        spdlog::info!("Dropped element buffer({})", self.ebo);
    }
}