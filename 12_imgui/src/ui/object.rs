use crate::{buffer::Buffer, errors, image::Image, program::Program, shader::Shader, texture::Texture, vertex_array::VertexArray};

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
    pub fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }
}

struct Border {
    size: [f32; 4],
    color: [Color; 4],
}

pub struct Object {
    // object
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub total_objects: usize,
    pub on_cursor_object: Option<usize>,
    pub prev_on_cursor_object: Option<usize>,

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
    pub cursor_pos: glm::Vec2,
    pub prev_cursor_pos: glm::Vec2,
    pub hiding: bool,
    pub moving: bool,
    pub sizing: [bool; 4],
    pub pressed: bool,
    pub mouse_on_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_off_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_down_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_up_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
}

impl Object {
    pub fn create(id: usize, ratio: glm::Vec2) -> Result<Rc<RefCell<Self>>, errors::Error> {
        // object
        let objects = Vec::new();
        let total_objects = 0;
        let on_cursor_object = None;
        let prev_on_cursor_object = None;

        // property
        let local_pos = glm::vec2(0.0, 0.0);
        let base_pos = glm::vec2(0.0, 0.0);
        let global_pos = base_pos + local_pos;
        let width = 0.0;
        let height = 0.0;
        let background_color = Color::from(1.0, 1.0, 1.0, 1.0);
        let padding = [0.0; 4];
        let border = Border { size: [0.0; 4], color: [Color::from(1.0, 1.0, 1.0, 1.0); 4] };
        let margin = [0.0; 4];

        // shader
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
            -1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 1.0,
            -1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (border.size[0] + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 1.0,
            -1.0 + (border.size[3] + padding[3]) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 0.0,
            -1.0 + (width - (border.size[1] + padding[1])) * ratio.x, 1.0 - (height - (border.size[2] + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 0.0,
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

        // event
        let cursor_pos = glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        let hiding = false;
        let moving = false;
        let sizing = [false; 4];
        let pressed = false;
        let mouse_on_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_off_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_down_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_up_event = Rc::new(RefCell::new(|_: &mut Self| {}));

        Ok(Rc::new(RefCell::new(Self { objects, total_objects, on_cursor_object, prev_on_cursor_object, id, ratio, width, height, local_pos, base_pos, global_pos, background_color, padding, border, margin,
            vertices_border, vertices_content, indices_border, indices_content, program, vao_border, vao_content, vbo_border, vbo_content, ebo_border, ebo_content, shader_type: ShaderType::Color, tbo: None,
            cursor_pos, prev_cursor_pos, hiding, moving, sizing, pressed, mouse_on_event, mouse_off_event, mouse_down_event, mouse_up_event })))
    }

    pub fn add_object(&mut self) -> Result<Rc::<RefCell::<Object>>, errors::Error> {
        let object = Object::create(self.total_objects, self.ratio)?;
        object.borrow_mut().set_base_pos(Some(self.global_pos.x), Some(self.global_pos.y));
        self.objects.push(object.clone());
        self.total_objects += 1;
        Ok(object)
    }

    pub fn delete_object(&mut self, id: usize) -> &mut Self {
        let mut target_index = None;
        for (index, object) in self.objects.iter().map(|object|{ object.borrow() }).enumerate() {
            if object.id == id {
                target_index = Some(index);
            }
        }
        if let Some(target_index) = target_index {
            self.objects.remove(target_index);
            self.total_objects -= 1;
        }
        self
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
        // border
        self.vertices_border[7] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[21] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[28] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[35] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[42] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[49] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[50] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[57] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[71] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[77] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[78] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[99] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[106] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        // content
        self.vertices_content[9] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn set_background_color(&mut self, background_color: Color) -> &mut Self {
        self.background_color = background_color; 
        for i in 0..4 {
            self.vertices_content[9 * i + 3] = background_color.r;
            self.vertices_content[9 * i + 4] = background_color.g;
            self.vertices_content[9 * i + 5] = background_color.b;
            self.vertices_content[9 * i + 6] = background_color.a;
        }
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
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
        self.vertices_content[0] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn set_border_color(&mut self, top: Option<Color>, right: Option<Color>, bottom: Option<Color>, left: Option<Color>) -> &mut Self {
        if let Some(top) = top {
            self.border.color[0] = top;
            for i in 0..4 {
                self.vertices_border[i * 7 + 3] = top.r;
                self.vertices_border[i * 7 + 4] = top.g;
                self.vertices_border[i * 7 + 5] = top.b;
                self.vertices_border[i * 7 + 6] = top.a;
            }
        };
        if let Some(right) = right {
            self.border.color[1] = right;
            for i in 4..8 {
                self.vertices_border[i * 7 + 3] = right.r;
                self.vertices_border[i * 7 + 4] = right.g;
                self.vertices_border[i * 7 + 5] = right.b;
                self.vertices_border[i * 7 + 6] = right.a;
            }
        };
        if let Some(bottom) = bottom {
            self.border.color[2] = bottom;
            for i in 8..12 {
                self.vertices_border[i * 7 + 3] = bottom.r;
                self.vertices_border[i * 7 + 4] = bottom.g;
                self.vertices_border[i * 7 + 5] = bottom.b;
                self.vertices_border[i * 7 + 6] = bottom.a;
            }
        };
        if let Some(left) = left {
            self.border.color[3] = left;
            for i in 12..16 {
                self.vertices_border[i * 7 + 3] = left.r;
                self.vertices_border[i * 7 + 4] = left.g;
                self.vertices_border[i * 7 + 5] = left.b;
                self.vertices_border[i * 7 + 6] = left.a;
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
        self.vertices_content[0] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn reshape(&mut self) {
        // border
        self.vertices_border[7] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[14] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[15] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[21] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[22] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[28] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[29] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[35] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[42] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[49] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[50] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[56] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[57] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.size[1]) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        self.vertices_border[71] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[77] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[78] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[91] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[92] = 1.0 - self.border.size[0] * self.ratio.y;
        self.vertices_border[99] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[105] = -1.0 + self.border.size[3] * self.ratio.x;
        self.vertices_border[106] = 1.0 - (self.height - self.border.size[2]) * self.ratio.y;
        // content
        self.vertices_content[0] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.size[0] + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.size[3] + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.size[1] + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.size[2] + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        for object in &self.objects {
            object.borrow_mut().ratio = self.ratio;
            object.borrow_mut().reshape();
        }
    }

    pub fn render(&self) {
        if self.hiding {
            return;
        }
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(self.global_pos.x * self.ratio.x, -1.0 * self.global_pos.y * self.ratio.y, 0.0));
        let transform = model;
        self.program.use_();
        // border
        self.vao_border.bind();
        unsafe {
            self.program.set_uniform_matrix4fv("transform\0", &transform);
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

    pub fn set_mouse_on_event<F>(&mut self, mouse_on_event: F) -> &mut Self where F: FnMut(&mut Self) + 'static {
        self.mouse_on_event = Rc::new(RefCell::new(mouse_on_event));
        self
    }

    pub fn set_mouse_off_event<F>(&mut self, mouse_off_event: F) -> &mut Self where F: FnMut(&mut Self) + 'static {
        self.mouse_off_event = Rc::new(RefCell::new(mouse_off_event));
        self
    }

    pub fn set_mouse_down_event<F>(&mut self, mouse_down_event: F) -> &mut Self where F: FnMut(&mut Self) + 'static {
        self.mouse_down_event = Rc::new(RefCell::new(mouse_down_event));
        self
    }

    pub fn set_mouse_up_event<F>(&mut self, mouse_up_event: F) -> &mut Self where F: FnMut(&mut Self) + 'static {
        self.mouse_up_event = Rc::new(RefCell::new(mouse_up_event));
        
        self
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        self.on_cursor_object = None;
        for (index, mut object) in self.objects.iter().map(|object|{ object.borrow_mut() }).enumerate().rev() {
            if object.global_pos.x <= self.cursor_pos.x && self.cursor_pos.x < object.global_pos.x + object.width && object.global_pos.y <= self.cursor_pos.y && self.cursor_pos.y < object.global_pos.y + object.height {
                self.on_cursor_object = Some(index);
                object.on_cursor_pos_event(x, y);
                break;
            }
        }
        if self.on_cursor_object != self.prev_on_cursor_object {
            if self.on_cursor_object.is_some() {
                spdlog::info!("Object({}): mouse on", self.objects[self.on_cursor_object.unwrap()].borrow().id);
                self.objects[self.on_cursor_object.unwrap()].borrow_mut().mouse_on();
            }
            if self.prev_on_cursor_object.is_some() {
                spdlog::info!("Object({}): mouse off", self.objects[self.prev_on_cursor_object.unwrap()].borrow().id);
                self.objects[self.prev_on_cursor_object.unwrap()].borrow_mut().mouse_off();
            }
        }
        self.prev_on_cursor_object = self.on_cursor_object;
        self.prev_cursor_pos = self.cursor_pos;
    }

    pub fn mouse_on(&mut self) {
        spdlog::info!("Object({}): mouse on", self.id);
        (self.mouse_on_event.clone().borrow_mut())(self)
    }

    pub fn mouse_off(&mut self) {
        spdlog::info!("Object({}): mouse off", self.id);
        if self.prev_on_cursor_object.is_some() {
            self.objects[self.prev_on_cursor_object.unwrap()].borrow_mut().mouse_off();
        }
        (self.mouse_off_event.clone().borrow_mut())(self);
    }

    pub fn mouse_down(&mut self) {
        spdlog::info!("Object({}): mouse down", self.id);
        self.pressed = true;
        if let Some(on_cursor_object) = self.on_cursor_object {
            self.objects[on_cursor_object].borrow_mut().mouse_down();
            // self.to_front_object(self.on_cursor_object.unwrap())
        }
        (self.mouse_down_event.clone().borrow_mut())(self);
    }

    pub fn mouse_up(&mut self) {
        spdlog::info!("Object({}): mouse up", self.id);
        if let Some(on_cursor_object) = self.on_cursor_object {
            self.objects[on_cursor_object].borrow_mut().mouse_up();
        }
        (self.mouse_up_event.clone().borrow_mut())(self);
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

    pub fn set_texture(&mut self, image: &Image) -> &mut Self {
        if let Some(tbo) = &mut self.tbo {
            tbo.set_texture(image);
            self.set_size(Some(image.get_width() as f32), Some(image.get_height() as f32));
        }
        self
    }

    fn enable_texture(&mut self) -> &mut Self {
        if self.tbo.is_none() {
            self.tbo = Some(Texture::create());
            self.vao_content.set(2, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 7) as *const _);
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

    pub fn to_front_object(&mut self, index: usize) {
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
}