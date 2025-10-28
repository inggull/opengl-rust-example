pub mod object;

use crate::{errors, image};
use crate::ui::object::{Object, Color, ShaderType};

use nalgebra_glm as glm;
use std::{rc::Rc, cell::RefCell};

pub struct Manager {
    // window
    windows: Vec<Rc<RefCell<Object>>>,
    total_windows: usize,
    window_color: Color,
    window_frame_size: f32,
    window_frame_ratio: f32,
    window_frame_color: Color,
    on_cursor_window: Option<usize>,
    prev_on_cursor_window: Option<usize>,

    // event
    frame_buffer_size: glm::Vec2,
    ratio: glm::Vec2,
    cursor_pos: glm::Vec2,
    prev_cursor_pos: glm::Vec2,
    window_cursor_pos_gap: glm::Vec2,
    window_close: bool,

    // resource
    close_image: image::Image,
    minimize_image: image::Image,
    maximize_image: image::Image,

    // buttons
    close_button_width: f32,
    button_width: f32,
    button_height: f32,
}

impl Manager {
    pub fn create(frame_buffer_size_x: f32, frame_buffer_size_y: f32) -> Result<Manager, errors::Error> {
        // window
        let windows = Vec::new();
        let total_windows = 0;
        let window_color = Color::from_u8(32, 32, 32, 255);
        let window_frame_size = 8.0;
        let window_frame_ratio = 3.0;
        let window_frame_color = Color::from_u8(255, 255, 255, 255); // (160, 224, 224, 224)
        let on_cursor_window = None;
        let prev_on_cursor_window = None;

        // event
        let frame_buffer_size = glm::vec2(frame_buffer_size_x, frame_buffer_size_y);
        let ratio = glm::vec2(2.0 / frame_buffer_size_x, 2.0 / frame_buffer_size_y);
        let cursor_pos= glm::vec2(0.0, 0.0);
        let prev_cursor_pos = glm::vec2(0.0, 0.0);
        let window_cursor_pos_gap = glm::vec2(0.0, 0.0);
        let window_close = false;

        // resource
        let close_image = image::Image::load("resources/images/close.png")?;
        spdlog::info!("Loaded image file \"resources/images/close.png\" ({} x {}, {} channels)", close_image.get_width(), close_image.get_height(), close_image.get_channel_count());
        let maximize_image = image::Image::load("resources/images/maximize.png")?;
        spdlog::info!("Loaded image file \"resources/images/maximize.png\" ({} x {}, {} channels)", maximize_image.get_width(), maximize_image.get_height(), maximize_image.get_channel_count());
        let minimize_image = image::Image::load("resources/images/minimize.png")?;
        spdlog::info!("Loaded image file \"resources/images/minimize.png\" ({} x {}, {} channels)", minimize_image.get_width(), minimize_image.get_height(), minimize_image.get_channel_count());

        // buttons
        let close_button_width = window_frame_size * window_frame_ratio * 2.0;
        let button_width = window_frame_size * window_frame_ratio;
        let button_height = window_frame_size * window_frame_ratio;

        Ok(Manager { windows, total_windows, window_color, window_frame_size, window_frame_ratio, window_frame_color, on_cursor_window, prev_on_cursor_window,
            frame_buffer_size, ratio, cursor_pos, prev_cursor_pos, window_cursor_pos_gap, window_close,
            close_image, maximize_image, minimize_image, close_button_width, button_width, button_height })
    }

    pub fn add_window(&mut self) -> Result<Rc::<RefCell::<Object>>, errors::Error> {
        let window = Object::create(self.total_windows, self.ratio)?;

        // property
        window.borrow_mut().set_padding(Some(0.0), Some(0.0), Some(0.0), Some(0.0));
        window.borrow_mut().set_size(Some(self.frame_buffer_size.x / 4.0 + self.window_frame_size * 2.0), Some(self.frame_buffer_size.y / 4.0 + self.window_frame_size * (self.window_frame_ratio + 2.0)));
        println!("{} x {}", window.borrow().width, window.borrow().height);
        window.borrow_mut().set_loacl_pos(Some(self.frame_buffer_size.x / 8.0 * self.total_windows as f32), Some(self.frame_buffer_size.y / 8.0 * self.total_windows as f32));
        window.borrow_mut().set_background_color(self.window_color);
        window.borrow_mut().set_border_color(Some(self.window_frame_color), Some(self.window_frame_color), Some(self.window_frame_color), Some(self.window_frame_color));
        window.borrow_mut().set_border_size(Some(self.window_frame_size * (self.window_frame_ratio + 1.0)), Some(self.window_frame_size), Some(self.window_frame_size), Some(self.window_frame_size));

        // close button
        let close_button_pos = glm::vec2(window.borrow().width - self.close_button_width - self.window_frame_size, 0.0);
        let close_button_color = Color::from_u8(160, 64, 64, 255);
        let close_button_on_color = Color::from_u8(160, 32, 32, 255);
        let close_button_down_color = Color::from_u8(128, 32, 32, 255);
        let close_button = window.borrow_mut().add_object()?;
        close_button.borrow_mut().set_size(Some(self.close_button_width), Some(self.button_height)).set_loacl_pos(Some(close_button_pos.x), Some(close_button_pos.y)).set_background_color(close_button_color);
        close_button.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(close_button_color); });
        close_button.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(close_button_on_color); });
        close_button.borrow_mut().set_mouse_down_event(move |button|{ button.set_background_color(close_button_down_color); });
        close_button.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(close_button_on_color); });
        let close_texture = close_button.borrow_mut().add_object()?;
        close_texture.borrow_mut().set_background_color(Color::from_u8(255, 255, 255, 255)).set_shader_type(ShaderType::Mix).set_texture(&self.close_image).set_loacl_pos(Some(16.0), Some(4.0));

        // buttons
        let button_pos = glm::vec2(window.borrow().width - self.close_button_width - self.window_frame_size - self.button_width, 0.0);
        let button_color = Color::from_u8(0, 0, 0, 0);
        let button_on_color = Color::from_u8(255, 255, 255, 32);
        let button_down_color = Color::from_u8(0, 0, 0, 32);

        // maximize button
        let maximize_button = window.borrow_mut().add_object()?;
        maximize_button.borrow_mut().set_size(Some(self.button_width), Some(self.button_height)).set_loacl_pos(Some(button_pos.x), Some(button_pos.y)).set_background_color(button_color);
        maximize_button.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(button_color); });
        maximize_button.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(button_on_color); });
        maximize_button.borrow_mut().set_mouse_down_event(move |button|{ button.set_background_color(button_down_color); });
        maximize_button.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(button_on_color); });
        let maximize_texture = maximize_button.borrow_mut().add_object()?;
        maximize_texture.borrow_mut().set_background_color(Color::from_u8(0, 0, 0, 255)).set_shader_type(ShaderType::Mix).set_texture(&self.maximize_image).set_loacl_pos(Some(4.0), Some(4.0));

        // minimize button
        let minimize_button = window.borrow_mut().add_object()?;
        minimize_button.borrow_mut().set_size(Some(self.button_width), Some(self.button_height)).set_loacl_pos(Some(button_pos.x - self.button_width), Some(button_pos.y)).set_background_color(button_color);
        minimize_button.borrow_mut().set_mouse_off_event(move |button|{ button.set_background_color(button_color); });
        minimize_button.borrow_mut().set_mouse_on_event(move |button|{ button.set_background_color(button_on_color); });
        minimize_button.borrow_mut().set_mouse_down_event(move |button|{ button.set_background_color(button_down_color); });
        minimize_button.borrow_mut().set_mouse_up_event(move |button|{ button.set_background_color(button_on_color); });
        let minimize_texture = minimize_button.borrow_mut().add_object()?;
        minimize_texture.borrow_mut().set_background_color(Color::from_u8(0, 0, 0, 255)).set_shader_type(ShaderType::Mix).set_texture(&self.minimize_image).set_loacl_pos(Some(4.0), Some(4.0));

        self.windows.push(window.clone());
        self.total_windows += 1;
        Ok(window)
    }

    pub fn delete_window(&mut self, id: usize) -> &mut Self {
        let mut target_index = None;
        for (index, window) in self.windows.iter().map(|window|{ window.borrow() }).enumerate() {
            if window.id == id {
                target_index = Some(index);
            }
        }
        if let Some(target_index) = target_index {
            self.windows.remove(target_index);
            self.total_windows -= 1;
        }
        self
    }

    pub fn render(&self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        for window in self.windows.iter().map(|w|{ w.borrow_mut() }) {
            window.render();
        }
    }

    pub fn on_cursor_pos_event(&mut self, x: f32, y: f32) {
        self.cursor_pos = glm::vec2(x, y);
        if let Some(mut focused_window) = self.windows.last().map(|o| { o.borrow_mut() }) {
            if !focused_window.objects[0].borrow().pressed && !focused_window.objects[1].borrow().pressed && !focused_window.objects[2].borrow().pressed {
                let x = focused_window.global_pos.x;
                let y = focused_window.global_pos.y;
                let width = focused_window.width;
                let height = focused_window.height;
                if focused_window.moving {
                    focused_window.set_loacl_pos(Some(self.cursor_pos.x - self.window_cursor_pos_gap.x), Some(self.cursor_pos.y - self.window_cursor_pos_gap.y));
                }
                if focused_window.sizing[0] {
                    if self.window_frame_size * (self.window_frame_ratio + 2.0) < height - (self.cursor_pos.y - y - self.window_cursor_pos_gap.y) {
                        focused_window.set_size(None, Some(height - (self.cursor_pos.y - y - self.window_cursor_pos_gap.y)));
                        focused_window.set_loacl_pos(None, Some(self.cursor_pos.y - self.window_cursor_pos_gap.y));
                    } else {
                        focused_window.set_size(None, Some(self.window_frame_size * (self.window_frame_ratio + 2.0)));
                        focused_window.set_loacl_pos(None, Some(y + height - self.window_frame_size * (self.window_frame_ratio + 2.0)));
                    }
                }
                if focused_window.sizing[1] {
                    if self.window_frame_size * 2.0 + self.button_width * 4.0 < self.cursor_pos.x - x + self.window_cursor_pos_gap.x {
                        focused_window.set_size(Some(self.cursor_pos.x - x + self.window_cursor_pos_gap.x), None);
                    } else {
                        focused_window.set_size(Some(self.window_frame_size * 2.0 + self.button_width * 4.0), None);
                    }
                }
                if focused_window.sizing[2] {
                    if self.window_frame_size * (self.window_frame_ratio + 2.0) < self.cursor_pos.y - y + self.window_cursor_pos_gap.y {
                        focused_window.set_size(None, Some(self.cursor_pos.y - y + self.window_cursor_pos_gap.y));
                    } else {
                        focused_window.set_size(None, Some(self.window_frame_size * (self.window_frame_ratio + 2.0)));
                    }
                }
                if focused_window.sizing[3] {
                    if self.window_frame_size * 2.0 + self.button_width * 4.0 < width - (self.cursor_pos.x - x - self.window_cursor_pos_gap.x) {
                        focused_window.set_size(Some(width - (self.cursor_pos.x - x - self.window_cursor_pos_gap.x)), None);
                        focused_window.set_loacl_pos(Some(self.cursor_pos.x - self.window_cursor_pos_gap.x), None);
                    } else {
                        focused_window.set_size(Some(self.window_frame_size * 2.0 + self.button_width * 4.0), None);
                        focused_window.set_loacl_pos(Some(x + width - (self.window_frame_size * 2.0 + self.button_width * 4.0)), None);
                    }
                }
                if focused_window.sizing[0] || focused_window.sizing[1] || focused_window.sizing[2] || focused_window.sizing[3] {
                    let width = focused_window.width;
                    focused_window.objects[0].borrow_mut().set_loacl_pos(Some(width - self.close_button_width - self.window_frame_size), None);
                    focused_window.objects[1].borrow_mut().set_loacl_pos(Some(width - self.close_button_width - self.button_width - self.window_frame_size), None);
                    focused_window.objects[2].borrow_mut().set_loacl_pos(Some(width - self.close_button_width - self.button_width - self.button_width - self.window_frame_size), None);
                }
            }
        }
        self.on_cursor_window = None;
        for (index, mut window) in self.windows.iter().map(|object|{ object.borrow_mut() }).enumerate().rev() {
            if window.global_pos.x <= self.cursor_pos.x && self.cursor_pos.x < window.global_pos.x + window.width && window.global_pos.y <= self.cursor_pos.y && self.cursor_pos.y < window.global_pos.y + window.height {
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
                self.to_front_window(index);
                let mut on_cursor_window = self.windows.last().unwrap().borrow_mut();
                if on_cursor_window.global_pos.x + self.window_frame_size <= self.cursor_pos.x && self.cursor_pos.x < on_cursor_window.global_pos.x + on_cursor_window.width - self.window_frame_size && on_cursor_window.global_pos.y + self.window_frame_size <= self.cursor_pos.y && self.cursor_pos.y < on_cursor_window.global_pos.y + self.window_frame_size * (self.window_frame_ratio + 1.0) {
                    on_cursor_window.moving = true;
                    self.window_cursor_pos_gap = self.cursor_pos - on_cursor_window.global_pos;
                }
                if on_cursor_window.global_pos.y <= self.cursor_pos.y && self.cursor_pos.y < on_cursor_window.global_pos.y + self.window_frame_size {
                    on_cursor_window.sizing[0] = true;
                    self.window_cursor_pos_gap.y = self.cursor_pos.y - on_cursor_window.global_pos.y;
                }
                if on_cursor_window.global_pos.x + on_cursor_window.width - self.window_frame_size <= self.cursor_pos.x && self.cursor_pos.x < on_cursor_window.global_pos.x + on_cursor_window.width {
                    on_cursor_window.sizing[1] = true;
                    self.window_cursor_pos_gap.x = on_cursor_window.global_pos.x + on_cursor_window.width - self.cursor_pos.x;
                }
                if on_cursor_window.global_pos.y + on_cursor_window.height - self.window_frame_size <= self.cursor_pos.y && self.cursor_pos.y < on_cursor_window.global_pos.y + on_cursor_window.height {
                    on_cursor_window.sizing[2] = true;
                    self.window_cursor_pos_gap.y = on_cursor_window.global_pos.y + on_cursor_window.height - self.cursor_pos.y;
                }
                if on_cursor_window.global_pos.x <= self.cursor_pos.x && self.cursor_pos.x < on_cursor_window.global_pos.x + self.window_frame_size {
                    on_cursor_window.sizing[3] = true;
                    self.window_cursor_pos_gap.x = self.cursor_pos.x - on_cursor_window.global_pos.x;
                }
                on_cursor_window.mouse_down();
            } else {
                self.windows[index].borrow_mut().mouse_up();
            }
        }
        // 구조 개선 필요
        if !mouse_down {
            if let Some(focused_window) = self.windows.last() {
                let mut focused_window = focused_window.borrow_mut();
                if focused_window.objects[0].borrow().pressed {
                    self.window_close = true;
                }
                focused_window.moving = false;
                for i in 0..4 {
                    focused_window.sizing[i] = false;
                }
                for i in 0..3 {
                    focused_window.objects[i].borrow_mut().pressed = false;
                }
            }
            if self.window_close {
                self.windows.pop();
                self.total_windows -= 1;
                self.window_close = false;
                self.prev_on_cursor_window = None;
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
}