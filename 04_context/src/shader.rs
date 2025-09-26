use crate::{errors, common};
use glad::gl;

pub struct Shader {
    shader: u32,
}

impl Shader {
    pub fn create<S>(file_path: S, type_: u32) -> Result<Shader, errors::Error> where S: AsRef<str> {
        let shader;

        // Load shader file
        let text = std::fs::read(file_path.as_ref())?;
        let text_len = text.len() as i32;

        // Create and compile shader
        unsafe {
            shader = gl::CreateShader(type_); // 쉐이더 핸들을 정수 형태로 반환
            // 하나의 쉐이더에 여러 개의 소스 코드를 전달할 수 있다
            gl::ShaderSource(shader, 1, &text.as_ptr().cast(), &text_len);
            gl::CompileShader(shader);

            // Check compile error
            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_log = [0i8; 1024];
                let mut length = 0;
                gl::GetShaderInfoLog(shader, 1024, &mut length, info_log.as_mut_ptr());
                let reason = common::c_str_to_string(info_log.as_ptr()).unwrap_or("".to_owned());
                return Err(errors::Error::CompileShaderError(reason))
            }
        }

        Ok(Shader { shader })
    }

    pub fn get(&self) -> u32 {
        self.shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
        spdlog::info!("Dropped shader({})", self.shader);
    }
}