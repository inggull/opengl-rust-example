mod errors;
mod common;
mod shader;
mod program;
mod context;
mod vertex_array;
mod buffer;
mod texture;
mod image;
mod ui;

use glfw::Context;
use ui::{Manager, window::Window, object::{ShaderType, Object}};

const WINDOW_NAME: &'static str = "MyImGui";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

fn main() -> errors::Result<(), errors::Error> {
    inner_main().into()
}

fn inner_main() -> Result<(), errors::Error> {
    // Initialize glfw
    spdlog::info!("Initialize glfw");
    
    let mut glfw = glfw::init_no_callbacks()?; // From 트레이트 구현으로 errors::Error 타입으로 변환됨

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Create glfw window
    spdlog::info!("Create glfw window");
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_NAME, glfw::WindowMode::Windowed).ok_or(errors::Error::CreateWindowError)?;
    window.set_cursor_pos(WINDOW_WIDTH as f64 / 2.0, WINDOW_HEIGHT as f64 / 2.0);
    // window.set_key_callback(on_key_event);
    // window.set_framebuffer_size_callback(on_frame_buffer_size_event);
    window.set_all_polling(true);
    window.make_current();

    // Initialize glad
    spdlog::info!("Initialize glad");
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        let gl_version = common::c_str_to_string(gl::GetString(gl::VERSION).cast());
        if gl_version.is_none() {
            spdlog::error!("Failed to initialize glad");
            panic!();
        }
        spdlog::info!("Loaded OpenGL {}", gl_version.unwrap());
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32); // State-setting function
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
    }

    let mut context = context::Context::create()?;

    let close = image::Image::load("resources/images/close.png")?;
    spdlog::info!("Loaded image file \"resources/images/close.png\" ({} x {}, {} channels)", close.get_width(), close.get_height(), close.get_channel_count());
    let maximize = image::Image::load("resources/images/maximize.png")?;
    spdlog::info!("Loaded image file \"resources/images/maximize.png\" ({} x {}, {} channels)", maximize.get_width(), maximize.get_height(), maximize.get_channel_count());
    let minimize = image::Image::load("resources/images/minimize.png")?;
    spdlog::info!("Loaded image file \"resources/images/minimize.png\" ({} x {}, {} channels)", minimize.get_width(), minimize.get_height(), minimize.get_channel_count());

    let mut ui_manager = Manager::create(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    let window_1 = Window::create(1, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let window_2 = Window::create(2, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let window_3 = Window::create(3, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;

    ui_manager.push_window(window_1).push_window(window_2).push_window(window_3);

    ui_manager.windows[0].set_pos(0.0, 0.0).set_frame_color(128,224,255, 224).set_color(32, 32, 32, 255);
    ui_manager.windows[1].set_pos(100.0, 100.0).set_frame_color(0,192,255, 224).set_color(32, 32, 32, 255);
    ui_manager.windows[2].set_pos(200.0, 200.0).set_frame_color(32,128,255, 224).set_color(32, 32, 32, 255);

    let button_1 = Object::create(1, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let button_2 = Object::create(2, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let button_3 = Object::create(3, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;

    ui_manager.windows[0].push_object(button_1).push_object(button_2).push_object(button_3);

    let window_1_size_x = ui_manager.windows[0].get_size().x;
    ui_manager.windows[0].objects[0].set_loacl_pos(window_1_size_x + 6.0 - 48.0, 0.0).set_size(48.0, 24.0).set_color(192, 64, 64, 255);
    ui_manager.windows[0].objects[1].set_loacl_pos(window_1_size_x + 6.0 - 48.0 - 24.0, 0.0).set_size(24.0, 24.0).set_color(255, 255, 255, 0);
    ui_manager.windows[0].objects[2].set_loacl_pos(window_1_size_x + 6.0 - 48.0 - 24.0 - 24.0, 0.0).set_size(24.0, 24.0).set_color(255, 255, 255, 0);

    let texture_1 = Object::create(4, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let texture_2 = Object::create(5, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let texture_3 = Object::create(6, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;

    ui_manager.windows[0].objects[0].push_object(texture_1);
    ui_manager.windows[0].objects[1].push_object(texture_2);
    ui_manager.windows[0].objects[2].push_object(texture_3);

    ui_manager.windows[0].objects[0].objects[0].set_shader_type(ShaderType::Mix).set_texture(&close).set_size(16.0, 16.0).set_loacl_pos(16.0, 4.0).set_color(255, 255, 255, 255);
    ui_manager.windows[0].objects[1].objects[0].set_shader_type(ShaderType::Texture).set_texture(&maximize).set_size(16.0, 16.0).set_loacl_pos(4.0, 4.0).set_color(0, 0, 0, 255);
    ui_manager.windows[0].objects[2].objects[0].set_shader_type(ShaderType::Texture).set_texture(&minimize).set_size(16.0, 16.0).set_loacl_pos(4.0, 4.0).set_color(0, 0, 0, 255);

    ui_manager.windows[0].objects[0].set_mouse_on_event(|button| {
        button.set_color(208, 32, 32, 255);
    }).set_mouse_off_event(|button| {
        button.set_color(192, 64, 64, 255);
    }).set_mouse_down_event(|button| {
        button.set_color(176, 32, 32, 255);
    }).set_mouse_up_event(|button| {
        button.set_color(208, 32, 32, 255);
    });

    ui_manager.windows[0].objects[1].set_mouse_on_event(|button| {
        button.set_color(255, 255, 255, 64);
    }).set_mouse_off_event(|button| {
        button.set_color(0, 0, 0, 0);
    }).set_mouse_down_event(|button| {
        button.set_color(0, 0, 0, 64);
    }).set_mouse_up_event(|button| {
        button.set_color(255, 255, 255, 64);
    });

    ui_manager.windows[0].objects[2].set_mouse_on_event(|button| {
        button.set_color(255, 255, 255, 64);
    }).set_mouse_off_event(|button| {
        button.set_color(0, 0, 0, 0);
    }).set_mouse_down_event(|button| {
        button.set_color(0, 0, 0, 64);
    }).set_mouse_up_event(|button| {
        button.set_color(255, 255, 255, 64);
    });

    // Start main loop
    spdlog::info!("Start main loop");
    let mut time;
    let mut prev_time: f32 = 0.0;
    let mut delta_time;
    while !window.should_close() {
        time = glfw.get_time() as f32;
        delta_time = time - prev_time;
        prev_time = time;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                    ui_manager.on_frame_buffer_size_event(width as f32, height as f32);
                    context.on_frame_buffer_size_event(width, height);
                    on_frame_buffer_size_event(&mut window, width, height);
                }
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                    if action == glfw::Action::Release {
                        context.on_key_event(key, false);
                    } else {
                        context.on_key_event(key, true);
                    }
                    on_key_event(&mut window, key, scancode, action, modifiers);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    ui_manager.on_cursor_pos_event(x as f32, y as f32);
                    context.on_cursor_pos_event(x as f32, y as f32);
                    on_cursor_pos_event(&mut window, x, y);
                }
                glfw::WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                    if mouse_button == glfw::MouseButtonLeft {
                        if action == glfw::Action::Release {
                            ui_manager.on_mouse_down_event(false);
                        } else {
                            ui_manager.on_mouse_down_event(true);
                        }
                    } else if mouse_button == glfw::MouseButtonRight {
                        if action == glfw::Action::Release {
                            context.on_mouse_down_event(false);
                        } else {
                            context.on_mouse_down_event(true);
                        }
                    }
                    on_mouse_button_event(&mut window, mouse_button, action, modifiers);
                }
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // State-using function
        }

        context.render(time, delta_time);
        ui_manager.render();

        window.swap_buffers();
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Ok(())
}

fn on_frame_buffer_size_event(_: &mut glfw::Window, width: i32, height: i32) {
    spdlog::info!("FramebufferSize changed: {} x {}", width, height);
}

fn on_key_event(window: &mut glfw::Window, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action, modifiers: glfw::Modifiers) {
    spdlog::info!("key: {}, scancode: {}, action: {}, mods: {}{}{}",
        key as usize, scancode,
        match action {
            glfw::Action::Press => "Press",
            glfw::Action::Release => "Release",
            glfw::Action::Repeat => "Repeat",
        },
        if modifiers.contains(glfw::Modifiers::Control) { "C" } else { "-" },
        if modifiers.contains(glfw::Modifiers::Shift) { "S" } else { "-" },
        if modifiers.contains(glfw::Modifiers::Alt) { "A" } else { "-" },
    );

    if (key == glfw::Key::Escape) && (action == glfw::Action::Press) {
        window.set_should_close(true);
    }
}

fn on_cursor_pos_event(_: &mut glfw::Window, x: f64, y: f64) {
    spdlog::info!("CursurPosition changed: {} x {}", x, y);
}

fn on_mouse_button_event(_: &mut glfw::Window, mouse_button: glfw::MouseButton, action: glfw::Action, modifiers: glfw::Modifiers) {
    spdlog::info!("mouse: {}, action: {}, mods: {}{}{}",
    mouse_button as usize,
    match action {
        glfw::Action::Press => "Press",
        glfw::Action::Release => "Release",
        glfw::Action::Repeat => "Repeat",
    },
    if modifiers.contains(glfw::Modifiers::Control) { "C" } else { "-" },
    if modifiers.contains(glfw::Modifiers::Shift) { "S" } else { "-" },
    if modifiers.contains(glfw::Modifiers::Alt) { "A" } else { "-" },
    );
}