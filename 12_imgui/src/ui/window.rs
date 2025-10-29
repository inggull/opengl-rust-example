use crate::{errors, image::Image};
use crate::ui::object::{Object, Color, ShaderType};

use nalgebra_glm as glm;
use std::{cell::RefCell, rc::Rc};

pub struct Window {
    // property
    pub id: usize,
    pub name: String,
    pub content_width: f32,
    pub content_height: f32,
    pub content_border_size: f32,
    pub border_size: f32,
    pub border_ratio: f32,
    pub background_width: f32,
    pub background_height: f32,
    pub pos: glm::Vec2,

    // elements
    pub button_width: f32,
    pub elements: [Rc<RefCell<Object>>; 5], // background, content, close, maximize, minimize
    pub on_cursor_element: Option<usize>,
    pub prev_on_cursor_element: Option<usize>,

    // event
    pub ratio: glm::Vec2,
    pub moving: bool,
    pub sizing: [bool; 4],
    pub closed: bool,
    pub cursor_pos: glm::Vec2,
    pub prev_cursor_pos: glm::Vec2,
    pub frame_cursor_gap: glm::Vec2,
}

impl Window {
    pub fn create(id: usize, name: &str, frame_buffer_size_x: f32, frame_buffer_size_y: f32, pos_x: f32, pos_y: f32, close_image: &Image, maximize_image: &Image, minimize_image: &Image) -> Result<Rc::<RefCell::<Self>>, errors::Error> {
        let ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);

        // property
        let name = name.to_owned();
        let content_width = frame_buffer_size_x / 4.0;
        let content_height = frame_buffer_size_y / 4.0;
        let content_border_size = 1.0;
        let border_size = 8.0;
        let border_ratio = 4.0;
        let background_width = content_width + border_size * 2.0;
        let background_height = content_height + border_size * (border_ratio + 1.0);
        let pos = glm::vec2(pos_x, pos_y);

        // event
        let moving = false;
        let sizing = [false; 4];
        let closed = false;
        let cursor_pos = glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        let frame_cursor_gap = glm::vec2(0.0, 0.0);

        // background
        let background = Object::create(0, "background", ratio)?;
        background.borrow_mut().set_size(Some(background_width), Some(background_height));
        background.borrow_mut().set_border_size(Some(border_size), Some(border_size), Some(border_size), Some(border_size));
        background.borrow_mut().set_border_color(Some(Color::from_u8(192, 192, 192, 224)), Some(Color::from_u8(192, 192, 192, 224)), Some(Color::from_u8(192, 192, 192, 224)), Some(Color::from_u8(192, 192, 192, 224)));
        background.borrow_mut().set_base_pos(Some(pos_x), Some(pos_y));
        background.borrow_mut().set_local_pos(None, None);
        background.borrow_mut().set_background_color(Color::from_u8(192, 192, 192, 224));

        // content
        let content = Object::create(1, "content box", ratio)?;
        content.borrow_mut().set_size(Some(content_width + content_border_size * 2.0), Some(content_height + content_border_size * 2.0));
        content.borrow_mut().set_border_size(Some(1.0), Some(1.0), Some(1.0), Some(1.0));
        content.borrow_mut().set_border_color(Some(Color::from_u8(0, 0, 0, 255)), Some(Color::from_u8(255, 255, 255, 255)), Some(Color::from_u8(255, 255, 255, 255)), Some(Color::from_u8(0, 0, 0, 255)));
        content.borrow_mut().set_base_pos(Some(pos_x), Some(pos_y));
        content.borrow_mut().set_local_pos(Some(border_size - content_border_size), Some(border_size * border_ratio - content_border_size));
        content.borrow_mut().set_background_color(Color::from_u8(64, 64, 64, 255));

        let button_width = border_size * (border_ratio - 1.0);

        // close
        let close = Object::create(2, "close button", ratio)?;
        close.borrow_mut().set_size(Some(button_width * 2.0), Some(button_width));
        close.borrow_mut().set_base_pos(Some(pos_x), Some(pos_y));
        close.borrow_mut().set_local_pos(Some(background_width - border_size - button_width * 2.0), None);
        close.borrow_mut().set_background_color(Color::from_u8(160, 64, 64, 255));
        close.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(Color::from_u8(160, 64, 64, 255)); });
        close.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(Color::from_u8(160, 32, 32, 255)); });
        close.borrow_mut().set_mouse_down_event(move |button|{ button.set_background_color(Color::from_u8(128, 32, 32, 255)); });
        close.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(Color::from_u8(160, 32, 32, 255)); });
        let close_texture = close.borrow_mut().add_child("close image")?;
        close_texture.borrow_mut().set_background_color(Color::from_u8(255, 255, 255, 255)).set_shader_type(ShaderType::Mix).set_texture(&close_image).set_local_pos(Some(16.0), Some(4.0));

        // maximize
        let maximize = Object::create(3, "maximize button", ratio)?;
        maximize.borrow_mut().set_size(Some(button_width), Some(button_width));
        maximize.borrow_mut().set_base_pos(Some(pos_x), Some(pos_y));
        maximize.borrow_mut().set_local_pos(Some(background_width - border_size - button_width * 3.0), None);
        maximize.borrow_mut().set_background_color(Color::from_u8(255, 255, 255, 0));
        maximize.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(Color::from_u8(0, 0, 0, 0)); });
        maximize.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(Color::from_u8(255, 255, 255, 32)); });
        maximize.borrow_mut().set_mouse_down_event(move |button|{ button.set_background_color(Color::from_u8(0, 0, 0, 32)); });
        maximize.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(Color::from_u8(255, 255, 255, 32)); });
        let maximize_texture = maximize.borrow_mut().add_child("maximize image")?;
        maximize_texture.borrow_mut().set_background_color(Color::from_u8(0, 0, 0, 255)).set_shader_type(ShaderType::Mix).set_texture(&maximize_image).set_local_pos(Some(4.0), Some(4.0));

        // minimize
        let minimize = Object::create(4, "minimize button", ratio)?;
        minimize.borrow_mut().set_size(Some(button_width), Some(button_width));
        minimize.borrow_mut().set_base_pos(Some(pos_x), Some(pos_y));
        minimize.borrow_mut().set_local_pos(Some(background_width - border_size - button_width * 4.0), None);
        minimize.borrow_mut().set_background_color(Color::from_u8(255, 255, 255, 0));
        minimize.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(Color::from_u8(0, 0, 0, 0)); });
        minimize.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(Color::from_u8(255, 255, 255, 32)); });
        minimize.borrow_mut().set_mouse_down_event(move |button: &mut Object|{ button.set_background_color(Color::from_u8(0, 0, 0, 32)); });
        minimize.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(Color::from_u8(255, 255, 255, 32)); });
        let minimize_texture = minimize.borrow_mut().add_child("minimize image")?;
        minimize_texture.borrow_mut().set_background_color(Color::from_u8(0, 0, 0, 255)).set_shader_type(ShaderType::Mix).set_texture(&minimize_image).set_local_pos(Some(4.0), Some(4.0));

        // elements
        let elements = [background, content, close, maximize, minimize];
        let on_cursor_element = None;
        let prev_on_cursor_element = None;

        Ok(Rc::new(RefCell::new(Self { id, name, content_width, content_height, content_border_size, border_size, border_ratio, background_width, background_height, pos, button_width, elements, on_cursor_element, prev_on_cursor_element, ratio, cursor_pos, moving ,sizing, closed, prev_cursor_pos, frame_cursor_gap })))
    }

    pub fn set_pos(&mut self, x: Option<f32>, y: Option<f32>) -> &mut Self {
        if let Some(x) = x {
            self.pos.x = x;
        }
        if let Some(y) = y {
            self.pos.y = y;
        }
        for element in &self.elements {
            element.borrow_mut().set_base_pos(x, y);
        }
        self
    }

    pub fn set_content_size(&mut self, content_width: Option<f32>, content_height: Option<f32>) -> &mut Self {
        if let Some(content_width) = content_width {
            self.content_width = content_width;
            self.elements[1].borrow_mut().set_size(Some(content_width + (self.content_border_size * 2.0)), None);
            self.background_width = content_width + self.border_size * 2.0;
            self.elements[0].borrow_mut().set_size(Some(self.background_width), None);
        }
        if let Some(content_height) = content_height {
            self.content_height = content_height;
            self.elements[1].borrow_mut().set_size(None, Some(content_height + (self.content_border_size * 2.0)));
            self.background_height = content_height + self.border_size * (self.border_ratio + 1.0);
            self.elements[0].borrow_mut().set_size(None, Some(self.background_height));
        }
        self
    }

    pub fn set_background_size(&mut self, background_width: Option<f32>, background_height: Option<f32>) -> &mut Self {
        if let Some(background_width) = background_width {
            self.background_width = background_width;
            self.elements[0].borrow_mut().set_size(Some(self.background_width), None);
            self.content_width = background_width - self.border_size * 2.0;
            self.elements[1].borrow_mut().set_size(Some(self.content_width + (self.content_border_size * 2.0)), None);
        }
        if let Some(background_height) = background_height {
            self.background_height = background_height;
            self.elements[0].borrow_mut().set_size(None, Some(self.background_height));
            self.content_height = background_height - self.border_size * (self.border_ratio + 1.0);
            self.elements[1].borrow_mut().set_size(None, Some(self.content_height + (self.content_border_size * 2.0)));
        }
        self
    }

    pub fn reshape(&self) {
        for element in &self.elements {
            element.borrow_mut().ratio = self.ratio;
            element.borrow_mut().reshape();
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        for mut element in self.elements.iter().map(|w|{ w.borrow_mut() }) {
            element.render();
        }
    }


    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        let temp_width = self.background_width;
        let temp_height = self.background_height;
        if self.moving {
            self.set_pos(Some(self.cursor_pos.x - self.frame_cursor_gap.x), Some(self.cursor_pos.y - self.frame_cursor_gap.y));
        }
        if self.sizing[0] {
            if self.border_size * (self.border_ratio + 1.0) < temp_height - (self.cursor_pos.y - self.pos.y - self.frame_cursor_gap.y) {
                self.set_background_size(None, Some(temp_height - (self.cursor_pos.y - self.pos.y - self.frame_cursor_gap.y)));
                self.set_pos(None, Some(self.cursor_pos.y - self.frame_cursor_gap.y));
            } else {
                self.set_background_size(None, Some(self.border_size * (self.border_ratio + 1.0)));
                self.set_pos(None, Some(self.pos.y + temp_height - self.border_size * (self.border_ratio + 1.0)));
            }
        }
        if self.sizing[1] {
            if self.border_size * 2.0 + self.button_width * 4.0 < self.cursor_pos.x - self.pos.x + self.frame_cursor_gap.x {
                self.set_background_size(Some(self.cursor_pos.x - self.pos.x + self.frame_cursor_gap.x), None);
            } else {
                self.set_background_size(Some(self.border_size * 2.0 + self.button_width * 4.0), None);
            }
        }
        if self.sizing[2] {
            if self.border_size * (self.border_ratio + 1.0) < self.cursor_pos.y - self.pos.y + self.frame_cursor_gap.y {
                self.set_background_size(None, Some(self.cursor_pos.y - self.pos.y + self.frame_cursor_gap.y));
            } else {
                self.set_background_size(None, Some(self.border_size * (self.border_ratio + 1.0)));
            }
        }
        if self.sizing[3] {
            if self.border_size * 2.0 + self.button_width * 4.0 < temp_width - (self.cursor_pos.x - self.pos.x - self.frame_cursor_gap.x) {
                self.set_background_size(Some(temp_width - (self.cursor_pos.x - self.pos.x - self.frame_cursor_gap.x)), None);
                self.set_pos(Some(self.cursor_pos.x - self.frame_cursor_gap.x), None);
            } else {
                self.set_background_size(Some(self.border_size * 2.0 + self.button_width * 4.0), None);
                self.set_pos(Some(self.pos.x + temp_width - (self.border_size * 2.0 + self.button_width * 4.0)), None);
            }
        }
        if self.sizing[0] || self.sizing[1] || self.sizing[2] || self.sizing[3] {
            self.elements[2].borrow_mut().set_local_pos(Some(self.background_width - self.border_size - self.button_width * 2.0), None);
            self.elements[3].borrow_mut().set_local_pos(Some(self.background_width - self.border_size - self.button_width * 3.0), None);
            self.elements[4].borrow_mut().set_local_pos(Some(self.background_width - self.border_size - self.button_width * 4.0), None);
        }
        self.on_cursor_element = None;
        for (index, mut element) in self.elements.iter().map(|object|{ object.borrow_mut() }).enumerate().rev() {
            if element.global_pos.x <= self.cursor_pos.x && self.cursor_pos.x < element.global_pos.x + element.width && element.global_pos.y <= self.cursor_pos.y && self.cursor_pos.y < element.global_pos.y + element.height {
                self.on_cursor_element = Some(index);
                element.on_cursor_pos_event(x, y);
                break;
            }
        }
        if self.on_cursor_element != self.prev_on_cursor_element {
            if self.on_cursor_element.is_some() {
                self.elements[self.on_cursor_element.unwrap()].borrow_mut().mouse_on();
            }
            if self.prev_on_cursor_element.is_some() {
                self.elements[self.prev_on_cursor_element.unwrap()].borrow_mut().mouse_off();
            }
        }
        self.prev_on_cursor_element = self.on_cursor_element;
        self.prev_cursor_pos = self.cursor_pos;
    }

    pub fn mouse_on(&mut self) {
        spdlog::info!("{}: mouse on", self.name);
        if let Some(index) = self.on_cursor_element {
            self.elements[index].borrow_mut().mouse_on();
        }
    }

    pub fn mouse_off(&mut self) {
        spdlog::info!("{}: mouse off", self.name);
        if let Some(index) = self.on_cursor_element {
            self.elements[index].borrow_mut().mouse_off();
            self.on_cursor_element = None;
        }
    }

    pub fn mouse_down(&mut self) {
        spdlog::info!("{}: mouse down", self.name);
        if self.on_cursor_element == Some(0) {
            if self.pos.x + self.border_size <= self.cursor_pos.x && self.cursor_pos.x < self.pos.x + self.background_width - self.border_size && self.pos.y + self.border_size <= self.cursor_pos.y && self.cursor_pos.y < self.pos.y + self.border_size * self.border_ratio {
                self.moving = true;
                self.frame_cursor_gap = self.cursor_pos - self.pos;
            }
            if self.pos.y <= self.cursor_pos.y && self.cursor_pos.y < self.pos.y + self.border_size {
                self.sizing[0] = true;
                self.frame_cursor_gap.y = self.cursor_pos.y - self.pos.y;
            }
            if self.pos.x + self.background_width - self.border_size <= self.cursor_pos.x && self.cursor_pos.x < self.pos.x + self.background_width {
                self.sizing[1] = true;
                self.frame_cursor_gap.x = self.pos.x + self.background_width - self.cursor_pos.x;
            }
            if self.pos.y + self.background_height - self.border_size <= self.cursor_pos.y && self.cursor_pos.y < self.pos.y + self.background_height {
                self.sizing[2] = true;
                self.frame_cursor_gap.y = self.pos.y + self.background_height - self.cursor_pos.y;
            }
            if self.pos.x <= self.cursor_pos.x && self.cursor_pos.x < self.pos.x + self.border_size {
                self.sizing[3] = true;
                self.frame_cursor_gap.x = self.cursor_pos.x - self.pos.x;
            }
        }
        if let Some(index) = self.on_cursor_element {
            self.elements[index].borrow_mut().mouse_down();
        }
    }

    pub fn mouse_up(&mut self) {
        spdlog::info!("{}: mouse up", self.name);
        if self.on_cursor_element == Some(2) && self.elements[2].borrow().pressed {
            self.closed = true;
        }
        if let Some(index) = self.on_cursor_element {
            self.elements[index].borrow_mut().mouse_up();
        }
    }
}