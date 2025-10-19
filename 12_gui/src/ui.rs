use super::{errors, shader, program, vertex_array, buffer};
use nalgebra_glm as glm;

pub struct UiManager {
    pub window: Vec<Window>,
    prev_mouse_pos: glm::Vec2,
    mouse_pos: glm::Vec2,
}

impl UiManager {
    pub fn create() -> UiManager {
        let window=  Vec::<Window>::new();
        let prev_mouse_pos = glm::vec2(0.0, 0.0);
        let mouse_pos= glm::vec2(0.0, 0.0);
        UiManager { window, prev_mouse_pos, mouse_pos }
    }

    pub fn push_window(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> Result<(), errors::Error> {
        let window = Window::create(frame_buffer_size_x, frame_buffer_size_y)?;
        self.window.push(window);
        Ok(())
    }

    pub fn on_cursur_pos_event(&mut self, x: f32, y: f32) {
        self.mouse_pos = glm::vec2(x, y);
        for (index, window) in self.window.iter_mut().enumerate().rev() {
            if window.pos.x <= self.mouse_pos.x && self.mouse_pos.x <= window.pos.x + (window.size.x as f32) && window.pos.y <= self.mouse_pos.y && self.mouse_pos.y <= window.pos.y + (window.size.y as f32) {
                window.mouse_on();
                println!("window {}: mouse on", index);
            } else {
                window.mouse_off();
                println!("window {}: mouse off", index);
            }

            if window.moving {
            window.pos.x += self.mouse_pos.x - self.prev_mouse_pos.x;
            window.pos.y += self.mouse_pos.y - self.prev_mouse_pos.y;
            }
        }
        self.prev_mouse_pos = self.mouse_pos;
    }

    pub fn on_mouse_down_event(&mut self, mouse_down: bool) {
        for (index, window) in self.window.iter_mut().enumerate().rev() {
            if window.pos.x <= self.mouse_pos.x && self.mouse_pos.x <= window.pos.x + (window.size.x as f32) && window.pos.y <= self.mouse_pos.y && self.mouse_pos.y <= window.pos.y + (window.size.y as f32) {
                if mouse_down {
                    window.mouse_down();
                    println!("window {}: mouse down", index);
                    let front_window = self.window.remove(index);
                    self.window.push(front_window);
                    return;
                } else {
                    window.mouse_up();
                    println!("window {}: mouse up", index);
                }
            }
        }
    }

    pub fn on_frame_buffer_size_event(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) {
        for window in &mut self.window {
            window.reshape(frame_buffer_size_x, frame_buffer_size_y);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        for window in &self.window {
            window.render();
        }
    }
}

pub struct Window {
    moving: bool,
    ratio: glm::Vec2,
    pos: glm::Vec3,
    size: glm::TVec2<u32>,
    color: glm::Vec4,
    vertices: [f32; 28],
    indices: [u32; 6],
    program: program::Program,
    vao: vertex_array::VertexArray,
    vbo: buffer::Buffer,
    ebo: buffer::Buffer,
}

impl Window {
    pub fn create(frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> Result<Window, errors::Error> {
        let ratio = glm::vec2(2.0 / frame_buffer_size_x as f32, 2.0 / frame_buffer_size_y as f32);
        let pos = glm::vec3(0.0, 0.0, 0.0);
        let size = glm::vec2(frame_buffer_size_x as u32 / 2, frame_buffer_size_y as u32 / 2);
        let color= glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 상단
            -1.0 + size.x as f32 * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 상단
            -1.0, 1.0 - size.y as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 하단
            -1.0 + size.x as f32 * ratio.x, 1.0 - size.y as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 하단
        ];
        let indices: [u32; 6] = [
            0, 1, 2,
            1, 2, 3,
        ];

        let vertex_shader = shader::Shader::create("shader/gui.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/gui.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());
        let program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());
        program.use_();
        let vao = vertex_array::VertexArray::create();
        vao.bind();
        let vbo = buffer::Buffer::create(gl::ARRAY_BUFFER, size_of_val(&vertices).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo = buffer::Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW);
        vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);

        Ok(Window { moving: false, ratio, pos, size, color, vertices, indices, program, vao, vbo, ebo })
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_size(&mut self, x: u32, y: u32) {
        self.size.x = x;
        self.size.y = y;
        self.resize();
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color.x = r as f32 / 255.0;
        self.color.y = g as f32 / 255.0;
        self.color.z = b as f32 / 255.0;
        self.color.w = a as f32 / 255.0;
        for index in 0..4 {
            self.vertices[index * 7 + 3] = self.color.x;
            self.vertices[index * 7 + 4] = self.color.y;
            self.vertices[index * 7 + 5] = self.color.z;
            self.vertices[index * 7 + 6] = self.color.w;
        }
        self.update();
    }

    pub fn reshape(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) {
        self.ratio = glm::vec2(2.0 / frame_buffer_size_x as f32, 2.0 / frame_buffer_size_y as f32);
        self.resize();
    }

    pub fn resize(&mut self) {
        self.vertices[7] = -1.0 + self.size.x as f32 * self.ratio.x;
        self.vertices[15] = 1.0 - self.size.y as f32 * self.ratio.y;
        self.vertices[21] = -1.0 + self.size.x as f32 * self.ratio.x;
        self.vertices[22] = 1.0 - self.size.y as f32 * self.ratio.y;
        self.update();
    }

    fn update(&self) {
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
    }

    pub fn mouse_on(&mut self) {

    }

    pub fn mouse_off(&mut self) {
        
    }

    pub fn mouse_down(&mut self) {
        self.moving = true;
    }

    pub fn mouse_up(&mut self) {
        self.moving = false;
    }

    pub fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.pos.x * self.ratio.x, -1.0 * self.pos.y * self.ratio.y, 0.0));
        let transform = model;
        unsafe {
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct Button {
    enable: bool,
    press: bool,
    moving: bool,
    ratio: glm::Vec2,
    pos: glm::Vec3,
    size: glm::TVec2<u32>,
    color: glm::Vec4,
    vertices: [f32; 28],
    indices: [u32; 6],
    program: program::Program,
    vao: vertex_array::VertexArray,
    vbo: buffer::Buffer,
    ebo: buffer::Buffer,
}

impl Button {
    pub fn create(frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> Result<Button, errors::Error> {
        let ratio = glm::vec2(2.0 / frame_buffer_size_x as f32, 2.0 / frame_buffer_size_y as f32);
        let pos = glm::vec3(0.0, 0.0, 0.0);
        let size = glm::vec2(frame_buffer_size_x as u32 / 2, frame_buffer_size_y as u32 / 2);
        let color= glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 상단
            -1.0 + size.x as f32 * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 상단
            -1.0, 1.0 - size.y as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 하단
            -1.0 + size.x as f32 * ratio.x, 1.0 - size.y as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 하단
        ];
        let indices: [u32; 6] = [
            0, 1, 2,
            1, 2, 3,
        ];

        let vertex_shader = shader::Shader::create("shader/gui.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = shader::Shader::create("shader/gui.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());
        let program = program::Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());
        program.use_();
        let vao = vertex_array::VertexArray::create();
        vao.bind();
        let vbo = buffer::Buffer::create(gl::ARRAY_BUFFER, size_of_val(&vertices).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo = buffer::Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW);
        vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);

        Ok(Button { enable: false, press: false, moving: false, ratio, pos, size, color, vertices, indices, program, vao, vbo, ebo })
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_size(&mut self, x: u32, y: u32) {
        self.size.x = x;
        self.size.y = y;
        self.resize();
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.color.x = r as f32 / 255.0;
        self.color.y = g as f32 / 255.0;
        self.color.z = b as f32 / 255.0;
        self.color.w = a as f32 / 255.0;
        for index in 0..4 {
            self.vertices[index * 7 + 3] = self.color.x;
            self.vertices[index * 7 + 4] = self.color.y;
            self.vertices[index * 7 + 5] = self.color.z;
            self.vertices[index * 7 + 6] = self.color.w;
        }
        self.update();
    }

    pub fn reshape(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) {
        self.ratio = glm::vec2(2.0 / frame_buffer_size_x as f32, 2.0 / frame_buffer_size_y as f32);
        self.resize();
    }

    pub fn resize(&mut self) {
        self.vertices[7] = -1.0 + self.size.x as f32 * self.ratio.x;
        self.vertices[15] = 1.0 - self.size.y as f32 * self.ratio.y;
        self.vertices[21] = -1.0 + self.size.x as f32 * self.ratio.x;
        self.vertices[22] = 1.0 - self.size.y as f32 * self.ratio.y;
        self.update();
    }

    fn update(&self) {
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
    }

    pub fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.pos.x * self.ratio.x, -1.0 * self.pos.y * self.ratio.y, 0.0));
        let transform = model;
        unsafe {
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}