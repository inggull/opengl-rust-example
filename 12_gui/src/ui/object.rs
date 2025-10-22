use crate::{errors, shader::Shader, program::Program, vertex_array::VertexArray, buffer::Buffer, texture::Texture, image::Image};
use nalgebra_glm as glm;
use std::rc::Rc;

pub struct Object {
    pub(super) id: usize,
    pub(super) ratio: glm::Vec2,
    pub(super) local_pos: glm::Vec2,
    pub(super) base_pos: glm::Vec2,
    pub(super) global_pos: glm::Vec2,
    pub(super) size: glm::Vec2,
    pub(super) color: glm::Vec4,
    pub(super) vertices: Vec<f32>,
    pub(super) indices: [u32; 6],
    pub(super) program: Program,
    pub(super) vao: VertexArray,
    pub(super) vbo: Buffer,
    pub(super) ebo: Buffer,
    pub(super) tbo: Option<Texture>,
    pub objects: Vec<Object>,
    pub(super) mouse_on_event: Rc<dyn Fn(&mut Self)>,
    pub(super) mouse_off_event: Rc<dyn Fn(&mut Self)>,
    pub(super) mouse_down_event: Rc<dyn Fn(&mut Self)>,
    pub(super) mouse_up_event: Rc<dyn Fn(&mut Self)>,
    offset: usize,
}

impl Object {
    pub fn create(id: usize, frame_buffer_size_x: f32, frame_buffer_size_y: f32) -> Result<Self, errors::Error> {
        let ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);
        let local_pos = glm::vec2(0.0, 0.0);
        let base_pos = glm::vec2(0.0, 0.0);
        let global_pos = base_pos + local_pos;
        let size = glm::vec2(20.0, 20.0);
        let color = glm::vec4(1.0, 1.0, 1.0, 1.0);
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, color.x, color.y, color.z, color.w, // 0: 좌측 상단
            -1.0 + size.x * ratio.x, 1.0, 0.0, color.x, color.y, color.z, color.w, // 1: 우측 상단
            -1.0, 1.0 - size.y * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 2: 좌측 하단
            -1.0 + size.x * ratio.x, 1.0 - size.y * ratio.y, 0.0, color.x, color.y, color.z, color.w, // 3: 우측 하단
        ];
        let vertices = vertices.to_vec();
        let indices: [u32; 6] = [
            0, 1, 2,
            1, 2, 3,
        ];

        let vertex_shader = Shader::create("shader/ui_object.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::create("shader/ui_object.frag", gl::FRAGMENT_SHADER)?;
        spdlog::info!("Created vertex shader({})", vertex_shader.get());
        spdlog::info!("Created fragment shader({})", fragment_shader.get());
        let program = Program::create(vec![&vertex_shader, &fragment_shader])?;
        spdlog::info!("Created program({})", program.get());
        program.use_();
        let vao = VertexArray::create();
        vao.bind();
        let vbo = Buffer::create(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()).cast_signed(), vertices.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo = Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices).cast_signed(), indices.as_ptr().cast(), gl::STATIC_DRAW);
        vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);

        let objects = Vec::<Object>::new();

        Ok(Self { id, ratio, local_pos, base_pos, global_pos, size, color, vertices, indices, program, vao, vbo, ebo, tbo: None, objects, mouse_on_event: Rc::new(|_| {}), mouse_off_event: Rc::new(|_| {}), mouse_down_event: Rc::new(|_| {}), mouse_up_event: Rc::new(|_| {}), offset: 0 })
    }

    pub fn enable_texture(&mut self) -> &mut Self {
        self.offset = 2;
        self.program.use_();
        let vertices: [f32; 36] = [
            -1.0, 1.0, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, 0.0, 1.0, // 0: 좌측 상단
            -1.0 + self.size.x * self.ratio.x, 1.0, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, 1.0, 1.0, // 1: 우측 상단
            -1.0, 1.0 - self.size.y * self.ratio.y, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, 0.0, 0.0, // 2: 좌측 하단
            -1.0 + self.size.x * self.ratio.x, 1.0 - self.size.y * self.ratio.y, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, 1.0, 0.0, // 3: 우측 하단
        ];
        self.vertices = vertices.to_vec();
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(self.vertices.as_slice()).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 0) as *const _);
        self.vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 3) as *const _);
        self.vao.set(2, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 7) as *const _);
        self.tbo = Some(Texture::create());
        self
    }

    pub fn disable_texture(&mut self) -> &mut Self {
        self.offset = 0;
        self.program.use_();
        let vertices: [f32; 28] = [
            -1.0, 1.0, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, // 0: 좌측 상단
            -1.0 + self.size.x * self.ratio.x, 1.0, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, // 1: 우측 상단
            -1.0, 1.0 - self.size.y * self.ratio.y, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, // 2: 좌측 하단
            -1.0 + self.size.x * self.ratio.x, 1.0 - self.size.y * self.ratio.y, 0.0, self.color.x, self.color.y, self.color.z, self.color.w, // 3: 우측 하단
        ];
        self.vertices = vertices.to_vec();
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(self.vertices.as_slice()).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        self.vao.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);
        unsafe {
            gl::DisableVertexAttribArray(2);
            // gl::DeleteBuffers(1, &self.tbo.as_ref().unwrap().get());
        }
        self
    }

    pub fn push_object(&mut self, mut object: Object) -> &mut Self {
        object.set_base_pos(self.global_pos.x, self.global_pos.y);
        self.objects.push(object);
        self
    }

    pub fn set_loacl_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.local_pos.x = x;
        self.local_pos.y = y;
        self.global_pos = self.base_pos + self.local_pos;
        for object in &mut self.objects {
            object.set_base_pos(self.global_pos.x, self.global_pos.y);
        }
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
            self.vertices[index * (7 + self.offset) + 3] = self.color.x;
            self.vertices[index * (7 + self.offset) + 4] = self.color.y;
            self.vertices[index * (7 + self.offset) + 5] = self.color.z;
            self.vertices[index * (7 + self.offset) + 6] = self.color.w;
        }
        self.update();
        self
    }

    pub fn set_texture(&mut self, image: &Image) -> &mut Self {
        if let Some(tbo) = &mut self.tbo {
            tbo.set_texture(image);
        }
        self
    }

    pub(super) fn set_base_pos(&mut self, x: f32, y: f32) -> &mut Self {
        self.base_pos.x = x;
        self.base_pos.y = y;
        self.global_pos = self.base_pos + self.local_pos;
        for object in &mut self.objects {
            object.set_base_pos(self.global_pos.x, self.global_pos.y);
        }
        self
    }

    pub(super) fn reshape(&mut self) {
        self.vertices[1 * (7 + self.offset)] = -1.0 + self.size.x * self.ratio.x;
        self.vertices[2 * (7 + self.offset) + 1] = 1.0 - self.size.y * self.ratio.y;
        self.vertices[3 * (7 + self.offset)] = -1.0 + self.size.x * self.ratio.x;
        self.vertices[3 * (7 + self.offset) + 1] = 1.0 - self.size.y * self.ratio.y;
        self.update();

        for object in &mut self.objects {
            object.ratio = self.ratio;
            object.reshape();
        }
    }

    fn update(&mut self) -> &mut Self {
        self.vao.bind();
        self.vbo.set(gl::ARRAY_BUFFER, size_of_val(self.vertices.as_slice()).cast_signed(), self.vertices.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub(super) fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.global_pos.x * self.ratio.x, -1.0 * self.global_pos.y * self.ratio.y, 0.0));
        let transform = model;
        unsafe {
            self.vao.bind();
            self.program.use_();
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            if let Some(tbo) = &self.tbo {
                gl::ActiveTexture(gl::TEXTURE0);
                tbo.bind();
                self.program.set_uniform1i("texture0\0", 0);
                self.program.set_uniform1i("flag\0", 1);
            } else {
                self.program.set_uniform1i("flag\0", 0);
            }
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        }
        for object in &self.objects {
            object.render();
        }
    }

    pub fn set_mouse_on_event<F>(&mut self, mouse_on_event: F) -> &mut Self where F: Fn(&mut Self) + 'static {
        self.mouse_on_event = Rc::new(mouse_on_event);
        self
    }

    pub fn set_mouse_off_event<F>(&mut self, mouse_off_event: F) -> &mut Self where F: Fn(&mut Self) + 'static {
        self.mouse_off_event = Rc::new(mouse_off_event);
        self
    }

    pub fn set_mouse_down_event<F>(&mut self, mouse_down_event: F) -> &mut Self where F: Fn(&mut Self) + 'static {
        self.mouse_down_event = Rc::new(mouse_down_event);
        self
    }

    pub fn set_mouse_up_event<F>(&mut self, mouse_up_event: F) -> &mut Self where F: Fn(&mut Self) + 'static {
        self.mouse_up_event = Rc::new(mouse_up_event);
        self
    }

    pub(super) fn mouse_on(&mut self) {
        let mouse_on_event = self.mouse_on_event.clone();
        mouse_on_event(self);
    }

    pub(super) fn mouse_off(&mut self) {
        let mouse_off_event = self.mouse_off_event.clone();
        mouse_off_event(self);
    }

    pub(super) fn mouse_down(&mut self) {
        let mouse_down_event = self.mouse_down_event.clone();
        mouse_down_event(self);
    }

    pub(super) fn mouse_up(&mut self) {
        let mouse_up_event = self.mouse_up_event.clone();
        mouse_up_event(self);
    }

    pub fn get_size(&mut self) -> &glm::Vec2 {
        &self.size
    }
}