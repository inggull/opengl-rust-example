pub mod window;
pub mod object;

use crate::{errors, image::Image};
use crate::ui::{window::Window};

use nalgebra_glm as glm;
use std::{rc::Rc, cell::RefCell};

pub struct Manager {
    // window
    windows: Vec<Rc<RefCell<Window>>>,
    total_windows: usize,
    on_cursor_window: Option<usize>,
    prev_on_cursor_window: Option<usize>,

    // event
    frame_buffer_size: glm::Vec2,
    ratio: glm::Vec2,
    cursor_pos: glm::Vec2,
    prev_cursor_pos: glm::Vec2,

    // resource
    close_image: Image,
    maximize_image: Image,
    minimize_image: Image,
}

impl Manager {
    pub fn create(frame_buffer_size_x: f32, frame_buffer_size_y: f32) -> Result<Self, errors::Error> {
        // resource
        let close_image = Image::load("resources/images/close.png")?;
        spdlog::info!("Loaded image file \"resources/images/close.png\" ({} x {}, {} channels)", close_image.get_width(), close_image.get_height(), close_image.get_channel_count());
        let maximize_image = Image::load("resources/images/maximize.png")?;
        spdlog::info!("Loaded image file \"resources/images/maximize.png\" ({} x {}, {} channels)", maximize_image.get_width(), maximize_image.get_height(), maximize_image.get_channel_count());
        let minimize_image = Image::load("resources/images/minimize.png")?;
        spdlog::info!("Loaded image file \"resources/images/minimize.png\" ({} x {}, {} channels)", minimize_image.get_width(), minimize_image.get_height(), minimize_image.get_channel_count());

        // window
        let windows = Vec::new();
        let total_windows = 0;
        let on_cursor_window = None;
        let prev_on_cursor_window = None;

        // event
        let frame_buffer_size = glm::vec2(frame_buffer_size_x, frame_buffer_size_y);
        let ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);
        let cursor_pos= glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);

        Ok(Self { windows, total_windows, on_cursor_window, prev_on_cursor_window, frame_buffer_size, ratio, cursor_pos, prev_cursor_pos, close_image, maximize_image, minimize_image })
    }

    pub fn add_window(&mut self, name: &str) -> Result<Rc::<RefCell::<Window>>, errors::Error> {
        let window = Window::create(self.total_windows, name, self.frame_buffer_size.x, self.frame_buffer_size.y, self.total_windows as f32 * self.frame_buffer_size.x / 8.0, self.total_windows as f32 * self.frame_buffer_size.y / 8.0, &self.close_image, &self.maximize_image, &self.minimize_image)?;
        self.windows.push(window.clone());
        self.total_windows += 1;
        Ok(window)
    }

    pub fn render(&mut self) {
        // 닫힌 창이 없는지 확인
        let mut indices = Vec::new();
        for (index, window) in self.windows.iter().enumerate() {
            if window.borrow().closed {
                indices.push(index);
            }
        }
        for index in indices {
            self.windows.remove(index);
            if self.prev_on_cursor_window == Some(index) {
                self.prev_on_cursor_window = None;
            }
        }
        // 깊이 테스트 없이, 뒤에 있는 오브젝트부터 렌더링
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        for window in self.windows.iter().map(|w|{ w.borrow_mut() }) {
            window.render();
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        if let Some(focused_window) = self.windows.last() {
            let mut focused_window = focused_window.borrow_mut();
            if focused_window.moving || focused_window.sizing[0] || focused_window.sizing[1] || focused_window.sizing[2] || focused_window.sizing[3] {
                focused_window.on_cursor_pos_event(x, y);
                self.prev_cursor_pos = self.cursor_pos;
                return;
            }
        }
        self.on_cursor_window = None;
        for (index, mut window) in self.windows.iter().map(|object|{ object.borrow_mut() }).enumerate().rev() {
            if window.pos.x <= self.cursor_pos.x && self.cursor_pos.x < window.pos.x + window.background_width && window.pos.y <= self.cursor_pos.y && self.cursor_pos.y < window.pos.y + window.background_height {
                self.on_cursor_window = Some(index);
                window.on_cursor_pos_event(x, y);
                break;
            }
        }
        if self.on_cursor_window != self.prev_on_cursor_window {
            if self.on_cursor_window.is_some() {
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().mouse_on();
            }
            if self.prev_on_cursor_window.is_some() {
                self.windows[self.prev_on_cursor_window.unwrap()].borrow_mut().mouse_off();
            }
        }
        self.prev_on_cursor_window = self.on_cursor_window;
        self.prev_cursor_pos = self.cursor_pos;
    }

    pub fn on_mouse_down_event(&mut self, mouse_down: bool) {
        if let Some(index) = self.on_cursor_window {
            if mouse_down {
                self.windows[index].borrow_mut().mouse_down();
                self.bring_to_front(index);
                self.on_cursor_window = Some(self.windows.len() - 1);
                self.prev_on_cursor_window = self.on_cursor_window;
            } else {
                self.windows[index].borrow_mut().mouse_up();
            }
        }
        if !mouse_down {
            if let Some(focused_window) = self.windows.last() {
                let mut focused_window = focused_window.borrow_mut();
                focused_window.moving = false;
                for i in 0..4 {
                    focused_window.sizing[i] = false;
                }
            }
        }
    }

    pub fn on_frame_buffer_size_event(&mut self, frame_buffer_size_x: f32, frame_buffer_size_y: f32) {
        self.ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);
        for mut window in self.windows.iter().map(|w|{ w.borrow_mut() }) {
            window.ratio = self.ratio;
            window.reshape();
        }
    }

    pub fn bring_to_front(&mut self, index: usize) {
        if 1 < self.windows.len() {
            let front_window = self.windows.remove(index);
            self.windows.push(front_window);
        }
    }
}