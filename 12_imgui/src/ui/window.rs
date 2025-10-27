use crate::{errors, shader::Shader, program::Program, vertex_array::VertexArray, buffer::Buffer};
use crate::ui::object::Object;
use nalgebra_glm as glm;
use std::{rc::Rc, cell::RefCell};

pub struct Window {
    pub(super) id: usize,
    pub(super) moving: bool,
    pub(super) sizing: [bool; 4],
    pub(super) ratio: glm::Vec2,
    pub(super) pos: glm::Vec2,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) color: glm::Vec4,
    pub(super) frame_size: f32,
    pub(super) frame_color: glm::Vec4,
    pub(super) vertices: [f32; 56],
    pub(super) indices: [u32; 12],
    pub(super) program: Program,
    pub(super) vao: VertexArray,
    pub(super) vbo: Buffer,
    pub(super) ebo: Buffer,
    pub(super) total_objects: usize,
    pub(super) objects: Vec<Rc<RefCell<Object>>>,
    pub(super) on_cursor_object: Option<usize>,
    pub(super) prev_on_cursor_object: Option<usize>,
    pub(super) cursor_pos: glm::Vec2,
    pub(super) prev_cursor_pos: glm::Vec2,
}

impl Window {
    pub fn create(id: usize, frame_buffer_size_x: f32, frame_buffer_size_y: f32) -> Result<Rc<RefCell<Self>>, errors::Error> {
        let ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);
        let pos = glm::vec2(0.0, 0.0);
        let width = frame_buffer_size_x / 2.0;
        let height = frame_buffer_size_y / 2.0;
        let frame_size: f32 = 6.0;
        let color = glm::vec4(1.0, 1.0, 1.0, 1.0);
        let frame_color = glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 56] = [
            // 윈도우 내용
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 0: 좌측 상단
            -1.0 + width * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 1: 우측 상단
            -1.0, 1.0 - height * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 2: 좌측 하단
            -1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 3: 우측 하단

            // 윈도우 테두리
            -1.0 - frame_size * ratio.x, 1.0 + frame_size * 5.0 * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 4: 테두리 좌측 상단
            -1.0 + (width + frame_size) * ratio.x, 1.0 + frame_size * 5.0 * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 5: 테두리 우측 상단
            -1.0 - frame_size * ratio.x, 1.0 - (height + frame_size) * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 6: 테두리 좌측 하단
            -1.0 + (width + frame_size) * ratio.x, 1.0 - (height + frame_size) * ratio.y, 0.0, frame_color.x, frame_color.y, frame_color.z, frame_color.w, // 7: 테두리 우측 하단
        ];
        let indices: [u32; 12] = [
            // 윈도우 테두리
            4, 5, 6,
            5, 6, 7,

            // 윈도우 내용
            0, 1, 2,
            1, 2, 3,
        ];

        let vertex_shader = Shader::create("shader/ui_window.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::create("shader/ui_window.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());
        let program = Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());
        program.use_();
        let vao = VertexArray::create();
        vao.bind();
        let vbo = Buffer::create(gl::ARRAY_BUFFER, size_of_val(&vertices).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo = Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW);
        vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);

        let objects = Vec::new();
        let on_cursor_object = None;
        let prev_on_cursor_object = None;
        let cursor_pos = glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);

        Ok(Rc::new(RefCell::new(Self { id, moving: false, sizing: [false; 4], ratio, pos, width, height, color, frame_size, frame_color, vertices, indices, program, vao, vbo, ebo, total_objects: 0, objects, on_cursor_object, prev_on_cursor_object, cursor_pos, prev_cursor_pos })))
    }

    pub fn add_object(&mut self) -> Result<Rc::<RefCell::<Object>>, errors::Error> {
        self.total_objects += 1;
        let object = Object::create(self.total_objects, self.ratio)?;
        object.borrow_mut().set_base_pos(Some(self.pos.x), Some(self.pos.y));
        self.objects.push(object.clone());
        Ok(object)
    }

    pub(super) fn to_front_object(&mut self, index: usize) {
        if 0 < self.objects.len() {
            let front_object = self.objects.remove(index);
            self.objects.push(front_object);

            if self.prev_on_cursor_object.is_some() {
                if index < self.prev_on_cursor_object.unwrap() {
                    self.prev_on_cursor_object = Some(self.prev_on_cursor_object.unwrap() - 1)
                } else if index == self.prev_on_cursor_object.unwrap() {
                    self.prev_on_cursor_object = Some(self.objects.len() - 1);
                }
            }

            if self.on_cursor_object.is_some() {
                if index < self.on_cursor_object.unwrap() {
                    self.on_cursor_object = Some(self.on_cursor_object.unwrap() - 1)
                } else if index == self.on_cursor_object.unwrap() {
                    self.on_cursor_object = Some(self.objects.len() - 1);
                }
            }
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        let delta_pos = self.cursor_pos - self.prev_cursor_pos;
        
        self.prev_cursor_pos = self.cursor_pos;

        self.on_cursor_object = None;
        for (index, object) in self.objects.iter().enumerate().rev() {
            if object.borrow().global_pos.x <= self.cursor_pos.x && self.cursor_pos.x <= object.borrow().global_pos.x + object.borrow().width && object.borrow().global_pos.y <= self.cursor_pos.y && self.cursor_pos.y <= object.borrow().global_pos.y + object.borrow().height {
                self.on_cursor_object = Some(index);
                break;
            }
        }

        if self.on_cursor_object != self.prev_on_cursor_object {
            if self.on_cursor_object.is_some() {
                spdlog::info!("Window({}).object({}): mouse on", self.id, self.objects[self.on_cursor_object.unwrap()].borrow().id);
                self.objects[self.on_cursor_object.unwrap()].borrow_mut().mouse_on();
            }

            if self.prev_on_cursor_object.is_some() {
                spdlog::info!("Window({}).object({}): mouse off", self.id, self.objects[self.prev_on_cursor_object.unwrap()].borrow().id);
                self.objects[self.prev_on_cursor_object.unwrap()].borrow_mut().mouse_off();
            }
        }

        self.prev_on_cursor_object = self.on_cursor_object;
    }

    pub fn on_mouse_down_event(&mut self, mouse_down: bool) {
        if mouse_down {
            if self.on_cursor_object.is_some() {
                // 윈도우 활성화
                spdlog::info!("Window({}).object({}): mouse down", self.id, self.objects[self.on_cursor_object.unwrap()].borrow().id);
                self.objects[self.on_cursor_object.unwrap()].borrow_mut().mouse_down();
                self.to_front_object(self.on_cursor_object.unwrap())
            } else {
                // 활성화된 윈도우 없음
            }
        } else {
            if self.on_cursor_object.is_some() {
                spdlog::info!("Window({}).object({}): mouse up", self.id, self.objects[self.on_cursor_object.unwrap()].borrow().id);
                self.objects[self.on_cursor_object.unwrap()].borrow_mut().mouse_up();
            }
            // 가장 맨 위의 윈도우
            if self.objects.last().is_some() {
                let focused_object = self.objects.last_mut().unwrap();
            }
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos.x = x;
        self.pos.y = y;
        for object in &mut self.objects {
            object.borrow_mut().set_base_pos(Some(x), Some(y));
        }
        self
    }

    pub fn add_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.pos.x += x;
        self.pos.y += y;
        for object in &self.objects {
            object.borrow_mut().set_base_pos(Some(self.pos.x), Some(self.pos.y));
        }
        self
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width = width;
        self.height = height;
        self.reshape();
        self
    }

    pub fn add_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.width += width;
        self.height += height;
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

    pub(super) fn reshape(&mut self) {
        // 윈도우 내용
        self.vertices[7] = -1.0 + self.width * self.ratio.x;
        self.vertices[15] = 1.0 - self.height * self.ratio.y;
        self.vertices[21] = -1.0 + self.width * self.ratio.x;
        self.vertices[22] = 1.0 - self.height * self.ratio.y;

        // 윈도우 테두리
        self.vertices[28] = -1.0 - self.frame_size * self.ratio.x;
        self.vertices[29] = 1.0 + self.frame_size * 5.0 * self.ratio.y;
        self.vertices[35] = -1.0 + (self.width + self.frame_size) * self.ratio.x;
        self.vertices[36] = 1.0 + self.frame_size * 5.0 * self.ratio.y;
        self.vertices[42] = -1.0 - self.frame_size * self.ratio.x;
        self.vertices[43] = 1.0 - (self.height + self.frame_size) * self.ratio.y;
        self.vertices[49] = -1.0 + (self.width + self.frame_size) * self.ratio.x;
        self.vertices[50] = 1.0 - (self.height + self.frame_size) * self.ratio.y;

        self.update();

        for object in &self.objects {
            object.borrow_mut().ratio = self.ratio;
            object.borrow_mut().reshape();
        }
    }

    pub(super) fn update(&mut self) -> &mut Self {
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub(super) fn mouse_on(&mut self) {
    }

    pub(super) fn mouse_off(&mut self) {
    }

    pub(super) fn mouse_down(&mut self, cursor_pos: glm::Vec2) {
        if self.pos.x + self.frame_size <= cursor_pos.x && cursor_pos.x < self.pos.x + self.frame_size + self.width && self.pos.y + self.frame_size <= cursor_pos.y && cursor_pos.y < self.pos.y + self.frame_size * 5.0 {
            self.moving = true;
        }
        if self.pos.y <= cursor_pos.y && cursor_pos.y < self.pos.y + self.frame_size {
            self.sizing[0] = true;
        }
        if self.pos.x + self.frame_size + self.width <= cursor_pos.x && cursor_pos.x < self.pos.x + self.width + self.frame_size * 2.0 {
            self.sizing[1] = true;
        }
        if self.pos.y + self.frame_size * 5.0 + self.height <= cursor_pos.y && cursor_pos.y < self.pos.y + self.height + self.frame_size * 6.0 {
            self.sizing[2] = true;
        }
        if self.pos.x <= cursor_pos.x && cursor_pos.x < self.pos.x + self.frame_size {
            self.sizing[3] = true;
        }
    }

    pub(super) fn mouse_up(&mut self) {
    }

    pub(super) fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3((self.frame_size + self.pos.x) * self.ratio.x, -1.0 * (self.pos.y + self.frame_size * 5.0) * self.ratio.y, 0.0));
        let transform = model;
        unsafe {
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        }
        for object in &self.objects{
            object.borrow().render();
        }
    }
}