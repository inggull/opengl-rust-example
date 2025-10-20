use super::{errors, shader, program, vertex_array, buffer};
use nalgebra_glm as glm;

pub struct UiManager {
    window: Vec<Window>,
    on_cursor_window: Option<usize>,
    prev_on_cursor_window: Option<usize>,
    frame_buffer_size: glm::TVec2<i32>,
    ratio: glm::Vec2,
    cursor_pos: glm::Vec2,
    prev_cursor_pos: glm::Vec2,
}

impl UiManager {
    pub fn create(frame_buffer_size_x: i32, frame_buffer_size_y: i32) -> UiManager {
        let window=  Vec::<Window>::new();
        let frame_buffer_size = glm::vec2(frame_buffer_size_x, frame_buffer_size_y);
        let ratio = glm::vec2(2.0 / frame_buffer_size.x as f32, 2.0 / frame_buffer_size.y as f32);
        let cursor_pos= glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        UiManager { window, on_cursor_window: None, prev_on_cursor_window: None, frame_buffer_size, ratio, cursor_pos, prev_cursor_pos }
    }

    pub fn push_window(&mut self, id: usize, width: i32, height: i32) -> Result<&mut Window, errors::Error> {
        let window = Window::create(id, width as f32, height as f32, self.ratio)?;
        self.window.push(window);
        Ok(self.window.last_mut().unwrap())
    }

    fn to_front_window(&mut self, index: usize) {
        if 0 < self.window.len() {
            let front_window = self.window.remove(index);
            self.window.push(front_window);

            if self.prev_on_cursor_window.is_some() {
                if index < self.prev_on_cursor_window.unwrap() {
                    self.prev_on_cursor_window = Some(self.prev_on_cursor_window.unwrap() - 1)
                } else if index == self.prev_on_cursor_window.unwrap() {
                    self.prev_on_cursor_window = Some(self.window.len() - 1);
                }
            }

            if self.on_cursor_window.is_some() {
                if index < self.on_cursor_window.unwrap() {
                    self.on_cursor_window = Some(self.on_cursor_window.unwrap() - 1)
                } else if index == self.on_cursor_window.unwrap() {
                    self.on_cursor_window = Some(self.window.len() - 1);
                }
            }
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        let delta_pos = self.cursor_pos - self.prev_cursor_pos;
        // 윈도우 이동 및 크기 조정
        if self.window.last().is_some() {
            let focused_window = self.window.last_mut().unwrap();
            if focused_window.moving {
                focused_window.pos.x += delta_pos.x;
                focused_window.pos.y += delta_pos.y;
            }
            if focused_window.sizing[0] && self.prev_cursor_pos.y < focused_window.pos.y + focused_window.frame_size {
                focused_window.size.y -= delta_pos.y;
                focused_window.pos.y += delta_pos.y;
                if focused_window.size.y < 0.0 {
                    focused_window.pos.y -= 0.0 - focused_window.size.y;
                    focused_window.size.y = 0.0;
                }
            }
            if focused_window.sizing[1] && focused_window.pos.x + focused_window.frame_size + focused_window.size.x <= self.prev_cursor_pos.x {
                focused_window.size.x += delta_pos.x;
                if focused_window.size.x < 0.0 {
                    focused_window.size.x = 0.0;
                }
            }
            if focused_window.sizing[2] && focused_window.pos.y + focused_window.size.y + focused_window.frame_size * 4.0 <= self.prev_cursor_pos.y {
                focused_window.size.y += delta_pos.y;
                if focused_window.size.y < 0.0 {
                    focused_window.size.y = 0.0;
                }
            }
            if focused_window.sizing[3] && self.prev_cursor_pos.x < focused_window.pos.x + focused_window.frame_size {
                focused_window.size.x -= delta_pos.x;
                focused_window.pos.x += delta_pos.x;
                if focused_window.size.x < 0.0 {
                    focused_window.pos.x -= 0.0 - focused_window.size.x;
                    focused_window.size.x = 0.0;
                }
            }
            focused_window.reshape();
        }
        
        self.prev_cursor_pos = self.cursor_pos;

        self.on_cursor_window = None;
        for (index, window) in self.window.iter().enumerate().rev() {
            if window.pos.x <= self.cursor_pos.x && self.cursor_pos.x <= window.pos.x + (window.size.x + window.frame_size * 2.0) && window.pos.y <= self.cursor_pos.y && self.cursor_pos.y <= window.pos.y + (window.size.y + window.frame_size * 5.0) {
                self.on_cursor_window = Some(index);
                break;
            }
        }

        if self.on_cursor_window != self.prev_on_cursor_window {
            if self.on_cursor_window.is_some() {
                spdlog::info!("Window({}): mouse on", self.window[self.on_cursor_window.unwrap()].id);
                self.window[self.on_cursor_window.unwrap()].mouse_on();
            }

            if self.prev_on_cursor_window.is_some() {
                spdlog::info!("Window({}): mouse off", self.window[self.prev_on_cursor_window.unwrap()].id);
                self.window[self.prev_on_cursor_window.unwrap()].mouse_off();
            }
        }

        self.prev_on_cursor_window = self.on_cursor_window;
    }

    pub fn on_mouse_down_event(&mut self, mouse_down: bool) {
        if mouse_down {
            if self.on_cursor_window.is_some() {
                // 윈도우 활성화
                spdlog::info!("Window({}): mouse down", self.window[self.on_cursor_window.unwrap()].id);
                self.window[self.on_cursor_window.unwrap()].mouse_down(self.cursor_pos);
                self.to_front_window(self.on_cursor_window.unwrap())
            } else {
                // 활성화된 윈도우 없음
            }
        } else {
            if self.on_cursor_window.is_some() {
                spdlog::info!("Window({}): mouse up", self.window[self.on_cursor_window.unwrap()].id);
                self.window[self.on_cursor_window.unwrap()].mouse_up();
            }
            // 가장 맨 위의 윈도우
            if self.window.last().is_some() {
                let focused_window = self.window.last_mut().unwrap();
                focused_window.moving = false;
                for i in 0..4 {
                    focused_window.sizing[i] = false;
                }
            }
        }
    }

    pub fn on_frame_buffer_size_event(&mut self, frame_buffer_size_x: i32, frame_buffer_size_y: i32) {
        self.frame_buffer_size.x = frame_buffer_size_x;
        self.frame_buffer_size.y = frame_buffer_size_y;
        self.ratio.x = 2.0 / self.frame_buffer_size.x as f32;
        self.ratio.y = 2.0 / self.frame_buffer_size.y as f32;
        for window in &mut self.window {
            window.ratio = self.ratio;
            window.reshape();
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
    id: usize,
    moving: bool,
    sizing: [bool; 4],
    ratio: glm::Vec2,
    pos: glm::Vec3,
    size: glm::TVec2<f32>,
    color: glm::Vec4,
    frame_size: f32,
    frame_color: glm::Vec4,
    vertices: [f32; 56],
    indices: [u32; 12],
    program: program::Program,
    vao: vertex_array::VertexArray,
    vbo: buffer::Buffer,
    ebo: buffer::Buffer,
}

impl Window {
    pub fn create(id: usize, width: f32, height: f32, ratio: glm::Vec2) -> Result<Window, errors::Error> {
        let pos = glm::vec3(0.0, 0.0, 0.0);
        let size = glm::vec2(width, height);
        let frame_size: f32 = 8.0;
        let color = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let frame_color = glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 56] = [
            // 윈도우 내용
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 0: 좌측 상단
            -1.0 + size.x * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 1: 우측 상단
            -1.0, 1.0 - size.y * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 2: 좌측 하단
            -1.0 + size.x * ratio.x, 1.0 - size.y * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 3: 우측 하단

            // 윈도우 테두리
            -1.0 - frame_size * ratio.x, 1.0 + frame_size * 4.0 * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 4: 테두리 좌측 상단
            -1.0 + (size.x + frame_size) * ratio.x, 1.0 + frame_size * 4.0 * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 5: 테두리 우측 상단
            -1.0 - frame_size * ratio.x, 1.0 - (size.y + frame_size) * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 6: 테두리 좌측 하단
            -1.0 + (size.x + frame_size) * ratio.x, 1.0 - (size.y + frame_size) * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 7: 테두리 우측 하단
        ];
        let indices: [u32; 12] = [
            // 윈도우 테두리
            4, 5, 6,
            5, 6, 7,

            // 윈도우 내용
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

        Ok(Window { id, moving: false, sizing: [false; 4], ratio, pos, size, color, frame_size, frame_color, vertices, indices, program, vao, vbo, ebo })
    }

    pub fn set_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos.x = x;
        self.pos.y = y;
        self
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.size.x = width;
        self.size.y = height;
        self.reshape();
        self
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
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
        self
    }

    pub fn set_frame_size(&mut self, width: f32) -> &mut Self {
        self.frame_size = width;
        self.reshape();
        self
    }

    pub fn set_frame_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.frame_color.x = r as f32 / 255.0;
        self.frame_color.y = g as f32 / 255.0;
        self.frame_color.z = b as f32 / 255.0;
        self.frame_color.w = a as f32 / 255.0;
        for index in 4..8 {
            self.vertices[index * 7 + 3] = self.frame_color.x;
            self.vertices[index * 7 + 4] = self.frame_color.y;
            self.vertices[index * 7 + 5] = self.frame_color.z;
            self.vertices[index * 7 + 6] = self.frame_color.w;
        }
        self.update();
        self
    }

    fn reshape(&mut self) {
        // 윈도우 내용
        self.vertices[7] = -1.0 + self.size.x * self.ratio.x;
        self.vertices[15] = 1.0 - self.size.y * self.ratio.y;
        self.vertices[21] = -1.0 + self.size.x * self.ratio.x;
        self.vertices[22] = 1.0 - self.size.y * self.ratio.y;

        // 윈도우 테두리
        self.vertices[28] = -1.0 - self.frame_size * self.ratio.x;
        self.vertices[29] = 1.0 + self.frame_size * 4.0 * self.ratio.y;
        self.vertices[35] = -1.0 + (self.size.x + self.frame_size) * self.ratio.x;
        self.vertices[36] = 1.0 + self.frame_size * 4.0 * self.ratio.y;
        self.vertices[42] = -1.0 - self.frame_size * self.ratio.x;
        self.vertices[43] = 1.0 - (self.size.y + self.frame_size) * self.ratio.y;
        self.vertices[49] = -1.0 + (self.size.x + self.frame_size) * self.ratio.x;
        self.vertices[50] = 1.0 - (self.size.y + self.frame_size) * self.ratio.y;

        self.update();
    }

    fn update(&mut self) -> &mut Self {
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    fn mouse_on(&mut self) {
    }

    fn mouse_off(&mut self) {
    }

    fn mouse_down(&mut self, cursor_pos: glm::Vec2) {
        if self.pos.x + self.frame_size <= cursor_pos.x && cursor_pos.x < self.pos.x + self.frame_size + self.size.x && self.pos.y + self.frame_size <= cursor_pos.y && cursor_pos.y < self.pos.y + self.frame_size * 4.0 {
            self.moving = true;
        }
        if self.pos.y <= cursor_pos.y && cursor_pos.y < self.pos.y + self.frame_size {
            self.sizing[0] = true;
        }
        if self.pos.x + self.frame_size + self.size.x <= cursor_pos.x && cursor_pos.x < self.pos.x + self.size.x + self.frame_size * 2.0 {
            self.sizing[1] = true;
        }
        if self.pos.y + self.frame_size * 4.0 + self.size.y <= cursor_pos.y && cursor_pos.y < self.pos.y + self.size.y + self.frame_size * 5.0 {
            self.sizing[2] = true;
        }
        if self.pos.x <= cursor_pos.x && cursor_pos.x < self.pos.x + self.frame_size {
            self.sizing[3] = true;
        }
    }

    fn mouse_up(&mut self) {
    }

    fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3((self.frame_size + self.pos.x) * self.ratio.x, -1.0 * (self.pos.y + self.frame_size * 4.0) * self.ratio.y, 0.0));
        let transform = model;
        unsafe {
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}