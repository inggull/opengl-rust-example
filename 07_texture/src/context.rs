use super::{errors, shader, program, vertex_array, buffer, texture, image};

pub struct Context {
    program: program::Program,
    vao: vertex_array::VertexArray,
    vbo: buffer::Buffer,
    ebo: buffer::Buffer,
    tbo1: texture::Texture,
    tbo2: texture::Texture,
}

impl Context {
    pub fn create() -> Result<Context, errors::Error> {
        let vertex_shader = shader::Shader::create("shader/texture.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/texture.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());

        let program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());

        let vertices: [f32; 32] = [
            0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            -0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let indices: [u32; 6] = [
            0, 1, 2, // 첫 번째 삼각형
            2, 3, 0, // 두 번째 삼각형
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

        let logo = image::Image::load("resources/images/logo.png")?;
        spdlog::info!("Loaded image file \"resources/images/logo.png\" ({} x {}, {} channels)", logo.get_width(), logo.get_height(), logo.get_channel_count());
        let rust = image::Image::load("resources/images/rust.jpg")?;
        spdlog::info!("Loaded image file \"resources/images/rust.jpg\" ({} x {}, {} channels)", rust.get_width(), rust.get_height(), rust.get_channel_count());

        let tbo1 = texture::Texture::create();
        tbo1.set_texture(&logo);
        let tbo2 = texture::Texture::create();
        tbo2.set_texture(&rust);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0); // 0번 텍스쳐를 활성화
            tbo1.bind(); // 사용할 tbo를 지정
            gl::ActiveTexture(gl::TEXTURE1); // 1번 텍스쳐를 활성화
            tbo2.bind(); // 사용할 tbo를 지정

            program.use_();  // 사용할 프로그램을 지정
            gl::Uniform1i(gl::GetUniformLocation(program.get(), c"texture0".as_ptr()), 0); // 프로그램의 전역 변수 `texture0`에 0을 할당
            gl::Uniform1i(gl::GetUniformLocation(program.get(), c"texture1".as_ptr()), 1); // 프로그램의 전역 변수 `texture1`에 1을 할당

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }

        Ok(Context { program, vao, vbo, ebo, tbo1, tbo2 })
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT); // State-using function
            self.program.use_(); // 사용할 프로그램을 지정
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}