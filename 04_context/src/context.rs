use super::{errors, shader, program};
use glad::gl;

pub struct Context {
    program: program::Program,
    vao: u32,
}

impl Context {
    pub fn create() -> Result<Context, errors::Error> {
        let program;
        let mut vao = 0;

        let vertex_shader = shader::Shader::create("shader/simple.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/simple.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());

        program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());

        
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);  // State-setting function

            gl::GenVertexArrays(1, &mut vao);  // 새로운 vao를 생성
            spdlog::info!("Created vertex array({})", vao);
            // 셰이더의 전역 변수에 값을 전달
            gl::UseProgram(program.get());  // 사용할 프로그램을 지정
            gl::Uniform3f(gl::GetUniformLocation(program.get(), c"global_position".as_ptr()), 0.0, 0.0, 0.0);
            gl::Uniform4f(gl::GetUniformLocation(program.get(), c"global_color".as_ptr()), 1.0, 1.0, 1.0, 1.0);
        }

        Ok(Context { program, vao })
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);  // State-using function
            gl::BindVertexArray(self.vao);  // 사용할 vao를 지정
            gl::UseProgram(self.program.get());
            gl::DrawArrays(gl::POINTS, 0, 1);
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
        spdlog::info!("Dropped vertex array({})", self.vao);
    }
}