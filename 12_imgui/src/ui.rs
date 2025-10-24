pub mod window;
pub mod object;

use crate::errors;
use crate::ui::window::Window;

use nalgebra_glm as glm;
use std::{rc::Rc, cell::RefCell};

pub struct Manager {
    total_windows: usize,
    windows: Vec<Rc<RefCell<Window>>>,
    on_cursor_window: Option<usize>,
    prev_on_cursor_window: Option<usize>,
    frame_buffer_size: glm::Vec2,
    ratio: glm::Vec2,
    cursor_pos: glm::Vec2,
    prev_cursor_pos: glm::Vec2,
}

impl Manager {
    pub fn create(frame_buffer_size_x: f32, frame_buffer_size_y: f32) -> Manager {
        let windows=  Vec::new();
        let frame_buffer_size = glm::vec2(frame_buffer_size_x, frame_buffer_size_y);
        let ratio = glm::vec2(2.0 / frame_buffer_size.x, 2.0 / frame_buffer_size.y);
        let cursor_pos= glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        Manager { total_windows: 0, windows, on_cursor_window: None, prev_on_cursor_window: None, frame_buffer_size, ratio, cursor_pos, prev_cursor_pos }
    }

    pub fn add_window(&mut self) -> Result<Rc::<RefCell::<Window>>, errors::Error> {
        self.total_windows += 1;
        let window = Window::create(self.total_windows, self.frame_buffer_size.x, self.frame_buffer_size.y)?;
        self.windows.push(window.clone());
        Ok(window)
    }

    fn to_front_window(&mut self, index: usize) {
        if 0 < self.windows.len() {
            let front_window = self.windows.remove(index);
            self.windows.push(front_window);

            if self.prev_on_cursor_window.is_some() {
                if index < self.prev_on_cursor_window.unwrap() {
                    self.prev_on_cursor_window = Some(self.prev_on_cursor_window.unwrap() - 1)
                } else if index == self.prev_on_cursor_window.unwrap() {
                    self.prev_on_cursor_window = Some(self.windows.len() - 1);
                }
            }

            if self.on_cursor_window.is_some() {
                if index < self.on_cursor_window.unwrap() {
                    self.on_cursor_window = Some(self.on_cursor_window.unwrap() - 1)
                } else if index == self.on_cursor_window.unwrap() {
                    self.on_cursor_window = Some(self.windows.len() - 1);
                }
            }
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        let delta_pos = self.cursor_pos - self.prev_cursor_pos;
        // 윈도우 이동 및 크기 조정
        if self.windows.last().is_some() {
            let mut focused_window = self.windows[self.windows.len() - 1].borrow_mut();
            let focused_window_size_x = focused_window.size.x;
            let focused_window_size_y = focused_window.size.y;
            if focused_window.moving {
                focused_window.add_pos(delta_pos.x, delta_pos.y);
            }
            if focused_window.sizing[0] && self.prev_cursor_pos.y < focused_window.pos.y + focused_window.frame_size {
                focused_window.add_size(0.0, -delta_pos.y);
                focused_window.add_pos(0.0, delta_pos.y);
                
                if focused_window_size_y < 0.0 {
                    focused_window.add_pos(0.0, focused_window_size_y);
                    focused_window.set_size(focused_window_size_x, 0.0);
                }
            }
            if focused_window.sizing[1] && focused_window.pos.x + focused_window.frame_size + focused_window_size_x <= self.prev_cursor_pos.x {
                focused_window.add_size(delta_pos.x, 0.0);
                if focused_window_size_x < 0.0 {
                    focused_window.set_size(0.0, focused_window_size_y);
                }
            }
            if focused_window.sizing[2] && focused_window.pos.y + focused_window_size_y + focused_window.frame_size * 5.0 <= self.prev_cursor_pos.y {
                focused_window.add_size(0.0, delta_pos.y);
                if focused_window_size_y < 0.0 {
                    focused_window.set_size(focused_window_size_x, 0.0);
                }
            }
            if focused_window.sizing[3] && self.prev_cursor_pos.x < focused_window.pos.x + focused_window.frame_size {
                focused_window.add_size(-delta_pos.x, 0.0);
                focused_window.add_pos(delta_pos.x, 0.0);
                if focused_window_size_x < 0.0 {
                    focused_window.add_pos(focused_window_size_x, 0.0);
                    focused_window.set_size(0.0, focused_window_size_y);
                }
            }
            focused_window.reshape();
        }
        
        self.prev_cursor_pos = self.cursor_pos;

        self.on_cursor_window = None;
        for (index, mut window) in self.windows.iter().map(|w|{ w.borrow_mut() }).enumerate().rev() {
            if window.pos.x <= self.cursor_pos.x && self.cursor_pos.x <= window.pos.x + (window.size.x + window.frame_size * 2.0) && window.pos.y <= self.cursor_pos.y && self.cursor_pos.y <= window.pos.y + (window.size.y + window.frame_size * 6.0) {
                self.on_cursor_window = Some(index);
                window.on_cursor_pos_event(x, y);
                break;
            }
        }

        if self.on_cursor_window != self.prev_on_cursor_window {
            if self.on_cursor_window.is_some() {
                spdlog::info!("Window({}): mouse on", self.windows[self.on_cursor_window.unwrap()].borrow().id);
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().mouse_on();
            }

            if self.prev_on_cursor_window.is_some() {
                spdlog::info!("Window({}): mouse off", self.windows[self.prev_on_cursor_window.unwrap()].borrow().id);
                self.windows[self.prev_on_cursor_window.unwrap()].borrow_mut().mouse_off();
            }
        }

        self.prev_on_cursor_window = self.on_cursor_window;
    }

    pub fn on_mouse_down_event(&mut self, mouse_down: bool) {
        if mouse_down {
            if self.on_cursor_window.is_some() {
                // 윈도우 활성화
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().on_mouse_down_event(mouse_down);
                spdlog::info!("Window({}): mouse down", self.windows[self.on_cursor_window.unwrap()].borrow().id);
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().mouse_down(self.cursor_pos);
                self.to_front_window(self.on_cursor_window.unwrap())
            } else {
                // 활성화된 윈도우 없음
            }
        } else {
            if self.on_cursor_window.is_some() {
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().on_mouse_down_event(mouse_down);
                spdlog::info!("Window({}): mouse up", self.windows[self.on_cursor_window.unwrap()].borrow().id);
                self.windows[self.on_cursor_window.unwrap()].borrow_mut().mouse_up();
            }
            // 가장 맨 위의 윈도우
            if self.windows.last().is_some() {
                let mut focused_window = self.windows[self.windows.len() - 1].borrow_mut();
                focused_window.moving = false;
                for i in 0..4 {
                    focused_window.sizing[i] = false;
                }
            }
        }
    }

    pub fn on_frame_buffer_size_event(&mut self, frame_buffer_size_x: f32, frame_buffer_size_y: f32) {
        self.frame_buffer_size.x = frame_buffer_size_x;
        self.frame_buffer_size.y = frame_buffer_size_y;
        self.ratio.x = 2.0 / self.frame_buffer_size.x;
        self.ratio.y = 2.0 / self.frame_buffer_size.y;
        for mut window in self.windows.iter().map(|w|{ w.borrow_mut() }) {
            window.ratio = self.ratio;
            window.reshape();
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        for window in self.windows.iter().map(|w|{ w.borrow_mut() }) {
            window.render();
        }
    }
}