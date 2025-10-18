use super::{errors, shader, program, vertex_array, buffer};
use nalgebra_glm as glm;

pub struct UiManager {
    window: Vec<Window>,
    button: Vec<Button>,
    prev_mouse_pos: glm::Vec2,
    mouse_pos: glm::Vec2,
    mouse_press: bool,
}

impl UiManager {
    pub fn create() -> UiManager {
        let window=  Vec::<Window>::new();
        let button=  Vec::<Button>::new();
        let prev_mouse_pos = glm::vec2(0.0, 0.0);
        let mouse_pos= glm::vec2(0.0, 0.0);
        UiManager { window, button, prev_mouse_pos, mouse_pos, mouse_press: false }
    }

    pub fn add_window(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> Result<(), errors::Error> {
        let window = Window::create(frame_buffer_size_x, frame_buffer_size_y)?;
        self.window.push(window);
        Ok(())
    }

    pub fn add_button(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> Result<(), errors::Error> {
        let button = Button::create(frame_buffer_size_x, frame_buffer_size_y)?;
        self.button.push(button);
        Ok(())
    }

    pub fn set_window_pos(&mut self, index: usize, x: f32, y: f32) {
        if 0 < index && self.window.len() <= index {
            self.window[index - 1].pos.x = x;
            self.window[index - 1].pos.y = y;
        }
    }

    pub fn set_window_size(&mut self, index: usize, x: u32, y: u32) {
        if 0 < index && self.window.len() <= index {
            self.window[index - 1].size.x = x;
            self.window[index - 1].size.y = y;
        }
    }

    pub fn set_window_color(&mut self, index: usize, r: u8, g: u8, b: u8, a: u8) {
        if 0 < index && self.window.len() <= index {
            self.window[index - 1].color.x = r as f32 / 255.0;
            self.window[index - 1].color.y = g as f32 / 255.0;
            self.window[index - 1].color.z = b as f32 / 255.0;
            self.window[index - 1].color.w = a as f32 / 255.0;
        }
    }

    pub fn set_button_pos(&mut self, index: usize, x: f32, y: f32) {
        if 0 < index && self.button.len() <= index {
            self.button[index - 1].pos.x = x;
            self.button[index - 1].pos.y = y;
        }
    }

    pub fn set_button_size(&mut self, index: usize, x: u32, y: u32) {
        if 0 < index && self.button.len() <= index {
            self.button[index - 1].resize(x, y);
        }
    }

    pub fn set_button_color(&mut self, index: usize, r: u8, g: u8, b: u8, a: u8) {
        if 0 < index && self.button.len() <= index {
            self.button[index - 1].color.x = r as f32 / 255.0;
            self.button[index - 1].color.y = g as f32 / 255.0;
            self.button[index - 1].color.z = b as f32 / 255.0;
            self.button[index - 1].color.w = a as f32 / 255.0;
        }
    }

    pub fn set_button_disable(&mut self, index: usize) {
        self.button[index - 1].enable = false;
    }

    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_pos = glm::vec2(x, y);
        for button in &mut self.button {
            if button.pos.x <= self.mouse_pos.x && self.mouse_pos.x <= button.pos.x + button.size.x as f32 && button.pos.y <= self.mouse_pos.y && self.mouse_pos.y <= button.pos.y + button.size.y as f32 {
                if self.mouse_press {
                    button.change_color(0, 0, 0, 255);
                } else {
                    button.change_color(128, 128, 128, 255);
                }
            } else {
                button.change_color(255, 255, 255, 255);
            }
        }
        if self.mouse_press == false {
            return;
        }
        if self.mouse_press {
            for window in &mut self.window {
                if window.moving {
                    window.pos.x += self.mouse_pos.x - self.prev_mouse_pos.x;
                    window.pos.y += self.mouse_pos.y - self.prev_mouse_pos.y;
                }
            }
        }
        self.prev_mouse_pos = self.mouse_pos;
    }

    pub fn set_mouse_press(&mut self, mouse_press: bool) {
        self.mouse_press = mouse_press;
        if mouse_press {
            for window in &mut self.window {
                if window.pos.x <= self.mouse_pos.x && self.mouse_pos.x <= window.pos.x + window.size.x as f32 && window.pos.y <= self.mouse_pos.y && self.mouse_pos.y <= window.pos.y + window.size.y as f32 {
                    window.moving = true;
                }
            }
            for button in &mut self.button {
                if button.pos.x <= self.mouse_pos.x && self.mouse_pos.x <= button.pos.x + button.size.x as f32 && button.pos.y <= self.mouse_pos.y && self.mouse_pos.y <= button.pos.y + button.size.y as f32 {
                    button.press = true;
                }
            }
            self.prev_mouse_pos = self.mouse_pos;
        } else {
            for window in &mut self.window {
                if window.moving {
                    window.moving = false;
                }
            }
            for button in &mut self.button {
                if button.press {
                    button.enable = true;
                    button.press = false;
                }
            }
        }
    }

    pub fn reshape(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) {
        for window in &mut self.window {
            window.reshape(frame_buffer_size_x, frame_buffer_size_y);
        }
        for button in &mut self.button {
            button.reshape(frame_buffer_size_x, frame_buffer_size_y);
        }
    }

    pub fn render(&self) {
        for window in &self.window {
            window.render();
        }
        for button in &self.button {
            button.render();
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
        let size = glm::vec2(1, 1);
        let color= glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 상단
            -1.0 + (size.x - 1) as f32 * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 상단
            -1.0, 1.0 - (size.y - 1) as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 하단
            -1.0 + (size.x - 1) as f32 * ratio.x, 1.0 - (size.y - 1) as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 하단
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

    pub fn change_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
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

    pub fn set_size(&mut self, x: u32, y: u32) {
        if 0 < x && 0 < y {
            self.size.x = x;
            self.size.y = y;
            self.resize();
        }
    }

    pub fn resize(&mut self) {
        self.vertices[7] = -1.0 + (self.size.x - 1) as f32 * self.ratio.x;
        self.vertices[15] = 1.0 - (self.size.y - 1) as f32 * self.ratio.y;
        self.vertices[21] = -1.0 + (self.size.x - 1) as f32 * self.ratio.x;
        self.vertices[22] = 1.0 - (self.size.y - 1) as f32 * self.ratio.y;
        self.update();
    }

    fn update(&self) {
        self.program.use_();
        self.vao.bind();
        self.vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        self.vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);
    }

    pub fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.pos.x * self.ratio.x, -1.0 * self.pos.y * self.ratio.y, 0.0));
        let transform = glm::Mat4::identity() * model;
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
        let size = glm::vec2(1, 1);
        let color= glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 상단
            -1.0 + (size.x - 1) as f32 * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 상단
            -1.0, 1.0 - (size.y - 1) as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 왼쪽 하단
            -1.0 + (size.x - 1) as f32 * ratio.x, 1.0 - (size.y - 1) as f32 * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 오른쪽 하단
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

    pub fn change_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
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

    pub fn set_size(&mut self, x: u32, y: u32) {
        if 0 < x && 0 < y {
            self.size.x = x;
            self.size.y = y;
            self.resize();
        }
    }

    pub fn resize(&mut self) {
        self.vertices[7] = -1.0 + (self.size.x - 1) as f32 * self.ratio.x;
        self.vertices[15] = 1.0 - (self.size.y - 1) as f32 * self.ratio.y;
        self.vertices[21] = -1.0 + (self.size.x - 1) as f32 * self.ratio.x;
        self.vertices[22] = 1.0 - (self.size.y - 1) as f32 * self.ratio.y;
        self.update();
    }

    fn update(&self) {
        self.program.use_();
        self.vao.bind();
        self.vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        self.vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);
    }

    pub fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.pos.x * self.ratio.x, -1.0 * self.pos.y * self.ratio.y, 0.0));
        let transform = glm::Mat4::identity() * model;
        unsafe {
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}