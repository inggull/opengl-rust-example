use super::{errors, common, shader};
use nalgebra_glm as glm;

pub struct Program {
    program: u32,
}

impl Program {
    pub fn create(shaders: Vec<&shader::Shader>) -> Result<Program, errors::Error> {
        let program;

        unsafe {
            program = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(program, shader.get());
            }
            gl::LinkProgram(program);

            // Check link error
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut info_log = [0i8; 1024];
                let mut length = 0;
                gl::GetProgramInfoLog(program, 1024, &mut length, info_log.as_mut_ptr());
                let reason = common::c_str_to_string(info_log.as_ptr()).unwrap_or("".to_owned());
                return Err(errors::Error::LinkProgramError(reason))
            }
        }

        Ok(Program { program })
    }

    pub fn get(&self) -> u32 {
        self.program
    }

    pub fn use_(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_uniform1i<S>(&self, name: S, value: i32) where S: AsRef<str> {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.program, name.as_ref().as_ptr().cast()), value); // 프로그램의 전역 변수에 값을 할당
        }
    }

    pub fn set_uniform_matrix4fv<S>(&self, name: S, value: &glm::TMat4<f32>) where S: AsRef<str> {
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, name.as_ref().as_ptr().cast()), 1, gl::FALSE, glm::value_ptr(value).as_ptr()); // 프로그램의 전역 변수에 4차원 형렬의 주소값을 할당
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
        spdlog::info!("Dropped program({})", self.program);
    }
}