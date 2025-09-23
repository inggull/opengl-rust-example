use super::{errors, common, shader};
use glad::gl;

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
                return Err(errors::Error::CompileProgramError(reason))
            }
        }

        Ok(Program { program })
    }

    pub fn get(&self) -> u32 {
        self.program
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