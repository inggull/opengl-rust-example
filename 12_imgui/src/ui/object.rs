use crate::{errors, shader::Shader, program::Program, vertex_array::VertexArray, buffer::Buffer, texture::Texture, image::Image};
use nalgebra_glm as glm;
use std::{rc::Rc, cell::RefCell};

#[derive(PartialEq, Eq)]
pub enum ShaderType {
    Color,
    Texture,
    Mix,
}

#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }
}

struct Border {
    size: [f32; 4],
    color: [Color; 4],
}

pub struct Object {
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub total_objects: usize,

    // property
    pub id: usize,
    pub ratio: glm::Vec2,
    pub width: f32,
    pub height: f32,
    pub local_pos: glm::Vec2,
    pub base_pos: glm::Vec2,
    pub global_pos: glm::Vec2,
    pub background_color: Color,
    pub padding: [f32; 4],
    pub border: Border,
    pub margin: [f32; 4],

    // shader
    pub vertices_border: [f32; 112],
    pub vertices_content: [f32; 36],
    pub indices_border: [u32; 24],
    pub indices_content: [u32; 6],
    pub program: Program,
    pub vao_border: VertexArray,
    pub vao_content: VertexArray,
    pub vbo_border: Buffer,
    pub vbo_content: Buffer,
    pub ebo_border: Buffer,
    pub ebo_content: Buffer,
    pub shader_type: ShaderType,
    pub tbo: Option<Texture>,

    // event
    pub mouse_on_event: Rc<dyn Fn(&mut Self)>,
    pub mouse_off_event: Rc<dyn Fn(&mut Self)>,
    pub mouse_down_event: Rc<dyn Fn(&mut Self)>,
    pub mouse_up_event: Rc<dyn Fn(&mut Self)>,
}

impl Object {
    pub fn create(id: usize, ratio: glm::Vec2) -> Result<Rc<RefCell<Self>>, errors::Error> {
        let config  = std::collections::HashMap::<String, String>::new();
        let local_pos = glm::vec2(0.0, 0.0);
        let base_pos = glm::vec2(0.0, 0.0);
        let global_pos = base_pos + local_pos;
        let width = 200.0;
        let height = 200.0;
        let background_color = Color::from_u8(160, 192, 224, 255);
        let margin = [0.0; 4];
        let border = Border { size: [20.0; 4], color: [Color::from_u8(192, 244, 160, 255), Color::from_u8(255, 244, 160, 255), Color::from_u8(192, 244, 160, 255), Color::from_u8(255, 244, 160, 255)] };
        let padding = [10.0, 20.0, 30.0, 40.0];

        // border top left: [-1.0, 1.0, 0.0];
        // border top right: [-1.0 + width * ratio.x, 1.0, 0.0];
        // border bottom left: [-1.0, 1.0 - height * ratio.y, 0.0];
        // border bottom right: [-1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0];

        // padding top left: [-1.0 + border.size[3] * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0];
        // padding top right: [-1.0 + (width - border.size[1]) * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0];
        // padding bottom left: [-1.0 + border.size[3] * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0];
        // padding bottom right: [-1.0 + (width - border.size[1]) * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0];

        // content top left: [-1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0];
        // content top right: [-1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0];
        // content bottom left: [-1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0];
        // content bottom right: [-1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0];

        let vertices_border: [f32; 112] = [
            // border top
            -1.0, 1.0, 0.0, border.color[0].r, border.color[0].g, border.color[0].b, border.color[0].a,
            -1.0 + width * ratio.x, 1.0, 0.0, border.color[0].r, border.color[0].g, border.color[0].b, border.color[0].a,
            -1.0 + border.size[3] * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0, border.color[0].r, border.color[0].g, border.color[0].b, border.color[0].a,
            -1.0 + (width - border.size[1]) * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0, border.color[0].r, border.color[0].g, border.color[0].b, border.color[0].a,

            // border right
            -1.0 + (width - border.size[1]) * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0, border.color[1].r, border.color[1].g, border.color[1].b, border.color[1].a,
            -1.0 + width * ratio.x, 1.0, 0.0, border.color[1].r, border.color[1].g, border.color[1].b, border.color[1].a,
            -1.0 + (width - border.size[1]) * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0, border.color[1].r, border.color[1].g, border.color[1].b, border.color[1].a,
            -1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0, border.color[1].r, border.color[1].g, border.color[1].b, border.color[1].a,

            // border bottom
            -1.0 + border.size[3] * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0, border.color[2].r, border.color[2].g, border.color[2].b, border.color[2].a,
            -1.0 + (width - border.size[1]) * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0, border.color[2].r, border.color[2].g, border.color[2].b, border.color[2].a,
            -1.0, 1.0 - height * ratio.y, 0.0, border.color[2].r, border.color[2].g, border.color[2].b, border.color[2].a,
            -1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0, border.color[2].r, border.color[2].g, border.color[2].b, border.color[2].a,

            // border left
            -1.0, 1.0, 0.0, border.color[3].r, border.color[3].g, border.color[3].b, border.color[3].a,
            -1.0 + border.size[3] * ratio.x, 1.0 - border.size[0] * ratio.y, 0.0, border.color[3].r, border.color[3].g, border.color[3].b, border.color[3].a,
            -1.0, 1.0 - height * ratio.y, 0.0, border.color[3].r, border.color[3].g, border.color[3].b, border.color[3].a,
            -1.0 + border.size[3] * ratio.x, 1.0 - (height - border.size[2]) * ratio.y, 0.0, border.color[3].r, border.color[3].g, border.color[3].b, border.color[3].a,
        ];

        let vertices_content: [f32; 36] = [
            // content
            -1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 0.0,
            -1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 1.0,
            -1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 0.0,
            -1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 1.0,
        ];

        let indices_border: [u32; 24] = [
            // border top
            0, 1, 2,
            1, 2, 3,

            // border right
            4, 5, 6,
            5, 6, 7,

            // border bottom
            8, 9, 10,
            9, 10, 11,

            // border left
            12, 13, 14,
            13, 14, 15,
        ];

        let indices_content: [u32; 6] = [
            // content
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

        let vao_border = VertexArray::create();
        vao_border.bind();
        let vbo_border = Buffer::create(gl::ARRAY_BUFFER, size_of_val(vertices_border.as_slice()).cast_signed(), vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo_border = Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices_border).cast_signed(), indices_border.as_ptr().cast(), gl::STATIC_DRAW);
        vao_border.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 0) as *const _);
        vao_border.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 7) as i32, (size_of::<f32>() * 3) as *const _);

        let vao_content = VertexArray::create();
        vao_content.bind();
        let vbo_content = Buffer::create(gl::ARRAY_BUFFER, size_of_val(vertices_content.as_slice()).cast_signed(), vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        let ebo_content = Buffer::create(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&indices_content).cast_signed(), indices_content.as_ptr().cast(), gl::STATIC_DRAW);
        vao_content.set(0, 3, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 0) as *const _);
        vao_content.set(1, 4, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 3) as *const _);
        vao_content.set(2, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 7) as *const _);

        let objects = Vec::new();

        Ok(Rc::new(RefCell::new(Self { objects, total_objects: 0, id, ratio, width, height, local_pos, base_pos, global_pos, background_color, padding, border, margin,
            vertices_border, vertices_content, indices_border, indices_content, program, vao_border, vao_content, vbo_border, vbo_content, ebo_border, ebo_content, shader_type: ShaderType::Color, tbo: None,
            mouse_on_event: Rc::new(|_| {}), mouse_off_event: Rc::new(|_| {}), mouse_down_event: Rc::new(|_| {}), mouse_up_event: Rc::new(|_| {}) })))
    }

    pub fn add_object(&mut self) -> Result<Rc::<RefCell::<Object>>, errors::Error> {
        self.total_objects += 1;
        let object = Object::create(self.total_objects, self.ratio)?;
        object.borrow_mut().set_base_pos(Some(self.global_pos.x), Some(self.global_pos.y));
        self.objects.push(object.clone());
        Ok(object)
    }

    pub fn set_loacl_pos(&mut self, x: Option<f32>, y: Option<f32>) -> &mut Self {
        if let Some(x) = x {
            self.local_pos.x = x;
            self.global_pos.x = self.base_pos.x + self.local_pos.x;
            for object in &self.objects {
                object.borrow_mut().set_base_pos(Some(self.global_pos.x), None);
            }
        }
        if let Some(y) = y {
            self.local_pos.y = y;
            self.global_pos.y = self.base_pos.y + self.local_pos.y;
            for object in &self.objects {
                object.borrow_mut().set_base_pos(None, Some(self.global_pos.y));
            }
        }
        self
    }

    pub fn set_base_pos(&mut self, x: Option<f32>, y: Option<f32>) -> &mut Self {
        if let Some(x) = x {
            self.base_pos.x = x;
            self.global_pos.x = self.base_pos.x + self.local_pos.x;
            for object in &self.objects {
                object.borrow_mut().set_base_pos(Some(self.global_pos.x), None);
            }
        }
        if let Some(y) = y {
            self.base_pos.y = y;
            self.global_pos.y = self.base_pos.y + self.local_pos.y;
            for object in &self.objects {
                object.borrow_mut().set_base_pos(None, Some(self.global_pos.y));
            }
        }
        self
    }

    pub fn set_size(&mut self, width: Option<f32>, height: Option<f32>) -> &mut Self {
        if let Some(width) = width {
            self.width = width;
        }
        if let Some(height) = height {
            self.height = height;
        }
        self.reshape();
        self
    }

    pub fn set_background_color(&mut self, background_color: Option<Color>) -> &mut Self {
        if let Some(background_color) = background_color {
            self.background_color = background_color; 
            for index in 3..7 {
                self.vertices_content[0 + index] = background_color.r;
                self.vertices_content[7 + index] = background_color.g;
                self.vertices_content[14 + index] = background_color.b;
                self.vertices_content[21 + index] = background_color.a;
            }
        }
        self
    }

    pub fn set_margin(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) -> &mut Self {
        if let Some(top) = top { self.margin[0] = top; };
        if let Some(right) = right { self.margin[1] = right; };
        if let Some(bottom) = bottom { self.margin[2] = bottom; };
        if let Some(left) = left { self.margin[3] = left; };
        self
    }

    pub fn set_border_size(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) -> &mut Self {
        if let Some(top) = top {self.border.size[0] = top; };
        if let Some(right) = right { self.border.size[1] = right; };
        if let Some(bottom) = bottom { self.border.size[2] = bottom; };
        if let Some(left) = left { self.border.size[3] = left; };
        // border
        self.vertices_border[14] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[15] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[21] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[22] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[28] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[29] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[42] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[56] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[57] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[91] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[92] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[105] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[106] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        // content
        self.vertices_content[112] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[113] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[121] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[122] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[130] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[131] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[139] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[140] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn set_border_color(&mut self, top: Option<Color>, right: Option<Color>, bottom: Option<Color>, left: Option<Color>) -> &mut Self {
        if let Some(top) = top {
            self.border.color[0] = top;
            for index in 3..7 {
                self.vertices_border[0 + index] = top.r;
                self.vertices_border[7 + index] = top.g;
                self.vertices_border[14 + index] = top.b;
                self.vertices_border[21 + index] = top.a;
            }
        };
        if let Some(right) = right {
            self.border.color[1] = right;
            for index in 3..7 {
                self.vertices_border[28 + index] = right.r;
                self.vertices_border[35 + index] = right.g;
                self.vertices_border[42 + index] = right.b;
                self.vertices_border[49 + index] = right.a;
            }
        };
        if let Some(bottom) = bottom {
            self.border.color[2] = bottom;
            for index in 3..7 {
                self.vertices_border[56 + index] = bottom.r;
                self.vertices_border[63 + index] = bottom.g;
                self.vertices_border[70 + index] = bottom.b;
                self.vertices_border[77 + index] = bottom.a;
            }
        };
        if let Some(left) = left {
            self.border.color[3] = left;
            for index in 3..7 {
                self.vertices_border[84 + index] = left.r;
                self.vertices_border[91 + index] = left.g;
                self.vertices_border[98 + index] = left.b;
                self.vertices_border[105 + index] = left.a;
            }
        };
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn set_padding(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) -> &mut Self {
        if let Some(top) = top { self.padding[0] = top; };
        if let Some(right) = right { self.padding[1] = right; };
        if let Some(bottom) = bottom { self.padding[2] = bottom; };
        if let Some(left) = left { self.padding[3] = left; };
        // content
        self.vertices_content[112] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[113] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[121] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[122] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[130] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[131] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[139] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[140] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn reshape(&mut self) {

        for object in &self.objects {
            object.borrow_mut().ratio = self.ratio;
            object.borrow_mut().reshape();
        }
    }

    pub fn render(&self) {
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.global_pos.x * self.ratio.x, -1.0 * self.global_pos.y * self.ratio.y, 0.0));
        let transform = model;
        self.program.use_();
        // border
        self.vao_border.bind();
        unsafe {
            self.program.set_uniform1i("shader_type\0", 0);
            gl::DrawElements(gl::TRIANGLES, 24, gl::UNSIGNED_INT, std::ptr::null());
        }
        //content
        self.vao_content.bind();
        unsafe {
            self.program.set_uniform_matrix4fv("transform\0", &transform);
            if self.shader_type == ShaderType::Color {
                self.program.set_uniform1i("shader_type\0", 0);
            } else {
                gl::ActiveTexture(gl::TEXTURE0);
                self.tbo.as_ref().unwrap().bind();
                self.program.set_uniform1i("texture0\0", 0);
                if self.shader_type == ShaderType::Texture {
                    self.program.set_uniform1i("shader_type\0", 1);
                } else {
                    self.program.set_uniform1i("shader_type\0", 2);
                }
            }
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        for object in &self.objects {
            object.borrow().render();
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

    pub fn mouse_on(&mut self) {
        (self.mouse_on_event.clone())(self)
    }

    pub fn mouse_off(&mut self) {
        (self.mouse_off_event.clone())(self);
    }

    pub fn mouse_down(&mut self) {
        (self.mouse_down_event.clone())(self);
    }

    pub fn mouse_up(&mut self) {
        (self.mouse_up_event.clone())(self);
    }

    pub fn set_shader_type(&mut self, shader_type: ShaderType) -> &mut Self {
        if shader_type == ShaderType::Color {
            self.disable_texture();
        } else {
            self.enable_texture();
        }
        self.shader_type = shader_type;
        self
    }

    fn enable_texture(&mut self) -> &mut Self {
        if self.tbo.is_none() {
            self.tbo = Some(Texture::create());
        }
        self
    }

    fn disable_texture(&mut self) -> &mut Self {
        if self.tbo.is_some() {
            self.vao_content.bind();
            unsafe {
                gl::DisableVertexAttribArray(2);
            }
            self.tbo = None; // 자동으로 소멸자 호출
        }
        self
    }

    pub fn set_texture(&mut self, image: &Image) -> &mut Self {
        if let Some(tbo) = &mut self.tbo {
            tbo.set_texture(image);
        }
        self
    }
}