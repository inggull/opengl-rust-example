use crate::{buffer::Buffer, errors, image::Image, program::Program, shader::Shader, texture::Texture, vertex_array::VertexArray};

use nalgebra_glm as glm;
use std::{cell::RefCell, rc::Rc};

pub struct Object {
    // child
    pub children: Vec<Rc<RefCell<Self>>>,
    pub total_children: usize,
    pub on_cursor_child: Option<usize>,
    pub prev_on_cursor_child: Option<usize>,

    // property
    pub id: usize,
    pub name: String,
    pub ratio: glm::Vec2,
    pub width: f32,
    pub height: f32,
    pub local_pos: glm::Vec2,
    pub base_pos: glm::Vec2,
    pub global_pos: glm::Vec2,
    pub background_color: Color,
    pub padding: [f32; 4],
    pub border: Border,

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
    pub closed: bool,
    pub mouse_on_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_off_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_down_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
    pub mouse_up_event: Rc<RefCell<dyn FnMut(&mut Self)>>,
}

impl Object {
    pub fn create(id: usize, name: &str, ratio: glm::Vec2) -> Result<Rc<RefCell<Self>>, errors::Error> {
        // children
        let children = Vec::new();
        let total_children = 0;
        let on_cursor_child = None;
        let prev_on_cursor_child = None;

        // property
        let name = name.to_owned();
        let local_pos = glm::vec2(0.0, 0.0);
        let base_pos = glm::vec2(0.0, 0.0);
        let global_pos = base_pos + local_pos;
        let width = 0.0;
        let height = 0.0;
        let background_color = Color::from(1.0, 1.0, 1.0, 1.0);
        let padding = [0.0; 4];
        let border = Border::new();

        // shader
        // border top left: [-1.0, 1.0, 0.0];
        // border top right: [-1.0 + width * ratio.x, 1.0, 0.0];
        // border bottom left: [-1.0, 1.0 - height * ratio.y, 0.0];
        // border bottom right: [-1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0];

        // padding top left: [-1.0 + border.left.0 * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0];
        // padding top right: [-1.0 + (width - border.right.0) * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0];
        // padding bottom left: [-1.0 + border.left.0 * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0];
        // padding bottom right: [-1.0 + (width - border.right.0) * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0];

        // content top left: [-1.0 + (border.left.0 + padding[3]) * ratio.x, 1.0 - (border.top.0 + padding[0]) * ratio.y, 0.0];
        // content top right: [-1.0 + (width - (border.right.0 + padding[1])) * ratio.x, 1.0 - (border.top.0 + padding[0]) * ratio.y, 0.0];
        // content bottom left: [-1.0 + (border.left.0 + padding[3]) * ratio.x, 1.0 - (height - (border.bottom.0 + padding[2])) * ratio.y, 0.0];
        // content bottom right: [-1.0 + (width - (border.right.0 + padding[1])) * ratio.x, 1.0 - (height - (border.bottom.0 + padding[2])) * ratio.y, 0.0];

        let vertices_border: [f32; 112] = [
            // border top
            -1.0, 1.0, 0.0, border.top.1.r, border.top.1.g, border.top.1.b, border.top.1.a,
            -1.0 + width * ratio.x, 1.0, 0.0, border.top.1.r, border.top.1.g, border.top.1.b, border.top.1.a,
            -1.0 + border.left.0 * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0, border.top.1.r, border.top.1.g, border.top.1.b, border.top.1.a,
            -1.0 + (width - border.right.0) * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0, border.top.1.r, border.top.1.g, border.top.1.b, border.top.1.a,

            // border right
            -1.0 + (width - border.right.0) * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0, border.right.1.r, border.right.1.g, border.right.1.b, border.right.1.a,
            -1.0 + width * ratio.x, 1.0, 0.0, border.right.1.r, border.right.1.g, border.right.1.b, border.right.1.a,
            -1.0 + (width - border.right.0) * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0, border.right.1.r, border.right.1.g, border.right.1.b, border.right.1.a,
            -1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0, border.right.1.r, border.right.1.g, border.right.1.b, border.right.1.a,

            // border bottom
            -1.0 + border.left.0 * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0, border.bottom.1.r, border.bottom.1.g, border.bottom.1.b, border.bottom.1.a,
            -1.0 + (width - border.right.0) * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0, border.bottom.1.r, border.bottom.1.g, border.bottom.1.b, border.bottom.1.a,
            -1.0, 1.0 - height * ratio.y, 0.0, border.bottom.1.r, border.bottom.1.g, border.bottom.1.b, border.bottom.1.a,
            -1.0 + width * ratio.x, 1.0 - height * ratio.y, 0.0, border.bottom.1.r, border.bottom.1.g, border.bottom.1.b, border.bottom.1.a,

            // border left
            -1.0, 1.0, 0.0, border.left.1.r, border.left.1.g, border.left.1.b, border.left.1.a,
            -1.0 + border.left.0 * ratio.x, 1.0 - border.top.0 * ratio.y, 0.0, border.left.1.r, border.left.1.g, border.left.1.b, border.left.1.a,
            -1.0, 1.0 - height * ratio.y, 0.0, border.left.1.r, border.left.1.g, border.left.1.b, border.left.1.a,
            -1.0 + border.left.0 * ratio.x, 1.0 - (height - border.bottom.0) * ratio.y, 0.0, border.left.1.r, border.left.1.g, border.left.1.b, border.left.1.a,
        ];

        let vertices_content: [f32; 36] = [
            // content
            -1.0 + (border.left.0 + padding[3]) * ratio.x, 1.0 - (border.top.0 + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 1.0,
            -1.0 + (width - (border.right.0 + padding[1])) * ratio.x, 1.0 - (border.top.0 + padding[0]) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 1.0,
            -1.0 + (border.left.0 + padding[3]) * ratio.x, 1.0 - (height - (border.bottom.0 + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 0.0, 0.0,
            -1.0 + (width - (border.right.0 + padding[1])) * ratio.x, 1.0 - (height - (border.bottom.0 + padding[2])) * ratio.y, 0.0, background_color.r, background_color.g, background_color.b, background_color.a, 1.0, 0.0,
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

        let vertex_shader = Shader::create("shader/ui.vert", gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::create("shader/ui.frag", gl::FRAGMENT_SHADER)?;
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

        let shader_type = ShaderType::Color;
        let tbo = None;

        // event
        let cursor_pos = glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        let hiding = false;
        let moving = false;
        let sizing = [false; 4];
        let pressed = false;
        let closed = false;
        let mouse_on_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_off_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_down_event = Rc::new(RefCell::new(|_: &mut Self| {}));
        let mouse_up_event = Rc::new(RefCell::new(|_: &mut Self| {}));

        Ok(Rc::new(RefCell::new(Self { children, total_children, on_cursor_child, prev_on_cursor_child, id, ratio, width, height, name, local_pos, base_pos, global_pos, background_color, padding, border,
            vertices_border, vertices_content, indices_border, indices_content, program, vao_border, vao_content, vbo_border, vbo_content, ebo_border, ebo_content, shader_type, tbo,
            cursor_pos, prev_cursor_pos, hiding, moving, sizing, pressed, closed, mouse_on_event, mouse_off_event, mouse_down_event, mouse_up_event })))
    }

    pub fn add_child(&mut self, name: &str) -> Result<Rc::<RefCell::<Self>>, errors::Error> {
        let child = Self::create(self.total_children, name, self.ratio)?;
        child.borrow_mut().set_base_pos(Some(self.global_pos.x), Some(self.global_pos.y));
        self.children.push(child.clone());
        self.total_children += 1;
        Ok(child)
    }

    pub fn set_local_pos(&mut self, x: Option<f32>, y: Option<f32>) -> &mut Self {
        if let Some(x) = x {
            self.local_pos.x = x;
            self.global_pos.x = self.base_pos.x + self.local_pos.x;
            for child in &self.children {
                child.borrow_mut().set_base_pos(Some(self.global_pos.x), None);
            }
        }
        if let Some(y) = y {
            self.local_pos.y = y;
            self.global_pos.y = self.base_pos.y + self.local_pos.y;
            for child in &self.children {
                child.borrow_mut().set_base_pos(None, Some(self.global_pos.y));
            }
        }
        self
    }

    pub fn set_base_pos(&mut self, x: Option<f32>, y: Option<f32>) -> &mut Self {
        if let Some(x) = x {
            self.base_pos.x = x;
            self.global_pos.x = self.base_pos.x + self.local_pos.x;
            for child in &self.children {
                child.borrow_mut().set_base_pos(Some(self.global_pos.x), None);
            }
        }
        if let Some(y) = y {
            self.base_pos.y = y;
            self.global_pos.y = self.base_pos.y + self.local_pos.y;
            for child in &self.children {
                child.borrow_mut().set_base_pos(None, Some(self.global_pos.y));
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
        self.vertices_border[21] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[28] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[35] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[42] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[49] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[50] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[57] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[71] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[77] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[78] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[99] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[106] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        // content
        self.vertices_content[9] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
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

    pub fn set_border_size(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) -> &mut Self {
        if let Some(top) = top {self.border.top.0 = top; };
        if let Some(right) = right { self.border.right.0 = right; };
        if let Some(bottom) = bottom { self.border.bottom.0 = bottom; };
        if let Some(left) = left { self.border.left.0 = left; };
        // border
        self.vertices_border[14] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[15] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[21] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[22] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[28] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[29] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[42] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[56] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[57] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[91] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[92] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[105] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[106] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        // content
        self.vertices_content[0] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn set_border_color(&mut self, top: Option<Color>, right: Option<Color>, bottom: Option<Color>, left: Option<Color>) -> &mut Self {
        if let Some(top) = top {
            self.border.top.1 = top;
            for i in 0..4 {
                self.vertices_border[i * 7 + 3] = top.r;
                self.vertices_border[i * 7 + 4] = top.g;
                self.vertices_border[i * 7 + 5] = top.b;
                self.vertices_border[i * 7 + 6] = top.a;
            }
        };
        if let Some(right) = right {
            self.border.right.1 = right;
            for i in 4..8 {
                self.vertices_border[i * 7 + 3] = right.r;
                self.vertices_border[i * 7 + 4] = right.g;
                self.vertices_border[i * 7 + 5] = right.b;
                self.vertices_border[i * 7 + 6] = right.a;
            }
        };
        if let Some(bottom) = bottom {
            self.border.bottom.1 = bottom;
            for i in 8..12 {
                self.vertices_border[i * 7 + 3] = bottom.r;
                self.vertices_border[i * 7 + 4] = bottom.g;
                self.vertices_border[i * 7 + 5] = bottom.b;
                self.vertices_border[i * 7 + 6] = bottom.a;
            }
        };
        if let Some(left) = left {
            self.border.left.1 = left;
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
        self.vertices_content[0] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        self
    }

    pub fn reshape(&mut self) {
        // border
        self.vertices_border[7] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[14] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[15] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[21] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[22] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[28] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[29] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[35] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[42] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[43] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[49] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[50] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[56] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[57] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[63] = -1.0 + (self.width - self.border.right.0) * self.ratio.x;
        self.vertices_border[64] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        self.vertices_border[71] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[77] = -1.0 + self.width * self.ratio.x;
        self.vertices_border[78] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[91] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[92] = 1.0 - self.border.top.0 * self.ratio.y;
        self.vertices_border[99] = 1.0 - self.height * self.ratio.y;
        self.vertices_border[105] = -1.0 + self.border.left.0 * self.ratio.x;
        self.vertices_border[106] = 1.0 - (self.height - self.border.bottom.0) * self.ratio.y;
        // content
        self.vertices_content[0] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[1] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[9] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[10] = 1.0 - (self.border.top.0 + self.padding[0]) * self.ratio.y;
        self.vertices_content[18] = -1.0 + (self.border.left.0 + self.padding[3]) * self.ratio.x;
        self.vertices_content[19] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vertices_content[27] = -1.0 + (self.width - (self.border.right.0 + self.padding[1])) * self.ratio.x;
        self.vertices_content[28] = 1.0 - (self.height - (self.border.bottom.0 + self.padding[2])) * self.ratio.y;
        self.vao_border.bind();
        self.vbo_border.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_border).cast_signed(), self.vertices_border.as_ptr().cast(), gl::STATIC_DRAW);
        self.vao_content.bind();
        self.vbo_content.set(gl::ARRAY_BUFFER, size_of_val(&self.vertices_content).cast_signed(), self.vertices_content.as_ptr().cast(), gl::STATIC_DRAW);
        for child in &self.children {
            child.borrow_mut().ratio = self.ratio;
            child.borrow_mut().reshape();
        }
    }

    pub fn render(&mut self) {
        let mut indices = Vec::new();
        for (index, child) in self.children.iter().enumerate() {
            if child.borrow().closed {
                indices.push(index);
            }
        }
        for index in indices {
            self.children.remove(index);
        }
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
        for child in &self.children {
            child.borrow_mut().render();
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        self.on_cursor_child = None;
        for (index, mut child) in self.children.iter().map(|child|{ child.borrow_mut() }).enumerate().rev() {
            if child.global_pos.x <= self.cursor_pos.x && self.cursor_pos.x < child.global_pos.x + child.width && child.global_pos.y <= self.cursor_pos.y && self.cursor_pos.y < child.global_pos.y + child.height {
                self.on_cursor_child = Some(index);
                child.on_cursor_pos_event(x, y);
                break;
            }
        }
        if self.on_cursor_child != self.prev_on_cursor_child {
            if self.on_cursor_child.is_some() {
                self.children[self.on_cursor_child.unwrap()].borrow_mut().mouse_on();
            }
            if self.prev_on_cursor_child.is_some() {
                self.children[self.prev_on_cursor_child.unwrap()].borrow_mut().mouse_off();
            }
        }
        self.prev_on_cursor_child = self.on_cursor_child;
        self.prev_cursor_pos = self.cursor_pos;
    }

    pub fn mouse_on(&mut self) {
        spdlog::info!("{}: mouse on", self.name);
        (self.mouse_on_event.clone().borrow_mut())(self)
    }

    pub fn mouse_off(&mut self) {
        spdlog::info!("{}: mouse off", self.name);
        self.pressed = false;
        if self.prev_on_cursor_child.is_some() {
            self.children[self.prev_on_cursor_child.unwrap()].borrow_mut().mouse_off();
            self.on_cursor_child = None;
        }
        (self.mouse_off_event.clone().borrow_mut())(self);
    }

    pub fn mouse_down(&mut self) {
        spdlog::info!("{}: mouse down", self.name);
        self.pressed = true;
        if let Some(on_cursor_child) = self.on_cursor_child {
            self.children[on_cursor_child].borrow_mut().mouse_down();
            self.bring_to_front(self.on_cursor_child.unwrap())
        }
        (self.mouse_down_event.clone().borrow_mut())(self);
    }

    pub fn mouse_up(&mut self) {
        spdlog::info!("{}: mouse up", self.name);
        self.pressed = false;
        if let Some(on_cursor_child) = self.on_cursor_child {
            self.children[on_cursor_child].borrow_mut().mouse_up();
        }
        (self.mouse_up_event.clone().borrow_mut())(self);
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

    pub fn enable_texture(&mut self) -> &mut Self {
        if self.tbo.is_some() {
            return self;
        }
        self.tbo = Some(Texture::create());
        self.vao_content.set(2, 2, gl::FLOAT, gl::FALSE, (size_of::<f32>() * 9) as i32, (size_of::<f32>() * 7) as *const _);
        self
    }

    pub fn disable_texture(&mut self) -> &mut Self {
        if self.tbo.is_none() {
            return self;
        }
        self.vao_content.bind();
        unsafe {
            gl::DisableVertexAttribArray(2);
        }
        self.tbo = None; // 자동으로 소멸자 호출
        self
    }

    pub fn bring_to_front(&mut self, index: usize) {
        if 0 < self.children.len() {
            let front_child = self.children.remove(index);
            self.children.push(front_child);

            if self.prev_on_cursor_child.is_some() {
                if index < self.prev_on_cursor_child.unwrap() {
                    self.prev_on_cursor_child = Some(self.prev_on_cursor_child.unwrap() - 1)
                } else if index == self.prev_on_cursor_child.unwrap() {
                    self.prev_on_cursor_child = Some(self.children.len() - 1);
                }
            }

            if self.on_cursor_child.is_some() {
                if index < self.on_cursor_child.unwrap() {
                    self.on_cursor_child = Some(self.on_cursor_child.unwrap() - 1)
                } else if index == self.on_cursor_child.unwrap() {
                    self.on_cursor_child = Some(self.children.len() - 1);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ShaderType {
    Color,
    Texture,
    Mix,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
    pub fn from(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }
}

#[derive(Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub fn new() -> Self {
        Self { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
    }
    pub fn from_4v(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }
    pub fn from_3v(top_bottom: f32, right: f32, left: f32) -> Self {
        Self { top: top_bottom, right, bottom: top_bottom, left }
    }
    pub fn from_2v(top_bottom: f32, right_left: f32) -> Self {
        Self { top: top_bottom, right: right_left, bottom: top_bottom, left: right_left }
    }
    pub fn from_1v(top_right_bottom_left: f32) -> Self {
        Self { top: top_right_bottom_left, right: top_right_bottom_left, bottom: top_right_bottom_left, left: top_right_bottom_left }
    }
    pub fn set_4v(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) {
        if let Some(top) = top { self.top = top; }
        if let Some(right) = right { self.right = right; }
        if let Some(bottom) = bottom { self.bottom = bottom; }
        if let Some(left) = left { self.left = left; }
    }
    pub fn set_3v(&mut self, top_bottom: Option<f32>, right: Option<f32>, left: Option<f32>) {
        if let Some(top_bottom) = top_bottom {
            self.top = top_bottom;
            self.bottom = top_bottom;
        }
        if let Some(right) = right { self.right = right; }
        if let Some(left) = left { self.left = left; }
    }
    pub fn set_2v(&mut self, top_bottom: Option<f32>, right_left: Option<f32>) {
        if let Some(top_bottom) = top_bottom {
            self.top = top_bottom;
            self.bottom = top_bottom;
        }
        if let Some(right_left) = right_left {
            self.right = right_left;
            self.left = right_left;
        }
    }
    pub fn set_1v(&mut self, top_right_bottom_left: f32) {
        self.top = top_right_bottom_left;
        self.right = top_right_bottom_left;
        self.bottom = top_right_bottom_left;
        self.left = top_right_bottom_left;
    }
}

#[derive(Clone, Copy)]
pub struct Border {
    pub top: (f32, Color),
    pub right: (f32, Color),
    pub bottom: (f32, Color),
    pub left: (f32, Color),
}

impl Border {
    pub fn new() -> Self {
        Self { top: (0.0, Color::new()), right: (0.0, Color::new()), bottom: (0.0, Color::new()), left: (0.0, Color::new()) }
    }
    pub fn from_4v(top: (f32, Color), right: (f32, Color), bottom: (f32, Color), left: (f32, Color)) -> Self {
        Self { top, right, bottom, left }
    }
    pub fn from_3v(top_bottom: (f32, Color), right: (f32, Color), left: (f32, Color)) -> Self {
        Self { top: top_bottom, right, bottom: top_bottom, left }
    }
    pub fn from_2v(top_bottom: (f32, Color), right_left: (f32, Color)) -> Self {
        Self { top: top_bottom, right: right_left, bottom: top_bottom, left: right_left }
    }
    pub fn from_1v(top_right_bottom_left: (f32, Color)) -> Self {
        Self { top: top_right_bottom_left, right: top_right_bottom_left, bottom: top_right_bottom_left, left: top_right_bottom_left }
    }
    pub fn set_4v(&mut self, top: Option<(f32, Color)>, right: Option<(f32, Color)>, bottom: Option<(f32, Color)>, left: Option<(f32, Color)>) {
        if let Some(top) = top { self.top = top; }
        if let Some(right) = right { self.right = right; }
        if let Some(bottom) = bottom { self.bottom = bottom; }
        if let Some(left) = left { self.left = left; }
    }
    pub fn set_3v(&mut self, top_bottom: Option<(f32, Color)>, right: Option<(f32, Color)>, left: Option<(f32, Color)>) {
        if let Some(top_bottom) = top_bottom {
            self.top = top_bottom;
            self.bottom = top_bottom;
        }
        if let Some(right) = right { self.right = right; }
        if let Some(left) = left { self.left = left; }
    }
    pub fn set_2v(&mut self, top_bottom: Option<(f32, Color)>, right_left: Option<(f32, Color)>) {
        if let Some(top_bottom) = top_bottom {
            self.top = top_bottom;
            self.bottom = top_bottom;
        }
        if let Some(right_left) = right_left {
            self.right = right_left;
            self.left = right_left;
        }
    }
    pub fn set_1v(&mut self, top_right_bottom_left: (f32, Color)) {
        self.top = top_right_bottom_left;
        self.right = top_right_bottom_left;
        self.bottom = top_right_bottom_left;
        self.left = top_right_bottom_left;
    }
    pub fn set_size_4v(&mut self, top: Option<f32>, right: Option<f32>, bottom: Option<f32>, left: Option<f32>) {
        if let Some(top) = top { self.top.0 = top; }
        if let Some(right) = right { self.right.0  = right; }
        if let Some(bottom) = bottom { self.bottom.0  = bottom; }
        if let Some(left) = left { self.left.0  = left; }
    }
    pub fn set_size_3v(&mut self, top_bottom: Option<f32>, right: Option<f32>, left: Option<f32>) {
        if let Some(top_bottom) = top_bottom {
            self.top.0  = top_bottom;
            self.bottom.0  = top_bottom;
        }
        if let Some(right) = right { self.right.0  = right; }
        if let Some(left) = left { self.left.0  = left; }
    }
    pub fn set_size_2v(&mut self, top_bottom: Option<f32>, right_left: Option<f32>) {
        if let Some(top_bottom) = top_bottom {
            self.top.0  = top_bottom;
            self.bottom.0  = top_bottom;
        }
        if let Some(right_left) = right_left {
            self.right.0  = right_left;
            self.left.0  = right_left;
        }
    }
    pub fn set_size_1v(&mut self, top_right_bottom_left: f32) {
        self.top.0  = top_right_bottom_left;
        self.right.0  = top_right_bottom_left;
        self.bottom.0  = top_right_bottom_left;
        self.left.0  = top_right_bottom_left;
    }
    pub fn set_color_4v(&mut self, top: Option<Color>, right: Option<Color>, bottom: Option<Color>, left: Option<Color>) {
        if let Some(top) = top { self.top.1 = top; }
        if let Some(right) = right { self.right.1  = right; }
        if let Some(bottom) = bottom { self.bottom.1  = bottom; }
        if let Some(left) = left { self.left.1  = left; }
    }
    pub fn set_color_3v(&mut self, top_bottom: Option<Color>, right: Option<Color>, left: Option<Color>) {
        if let Some(top_bottom) = top_bottom {
            self.top.1  = top_bottom;
            self.bottom.1  = top_bottom;
        }
        if let Some(right) = right { self.right.1  = right; }
        if let Some(left) = left { self.left.1  = left; }
    }
    pub fn set_color_2v(&mut self, top_bottom: Option<Color>, right_left: Option<Color>) {
        if let Some(top_bottom) = top_bottom {
            self.top.1  = top_bottom;
            self.bottom.1  = top_bottom;
        }
        if let Some(right_left) = right_left {
            self.right.1  = right_left;
            self.left.1  = right_left;
        }
    }
    pub fn set_color_1v(&mut self, top_right_bottom_left: Color) {
        self.top.1  = top_right_bottom_left;
        self.right.1  = top_right_bottom_left;
        self.bottom.1  = top_right_bottom_left;
        self.left.1  = top_right_bottom_left;
    }
}