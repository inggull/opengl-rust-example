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

const WINDOW_NAME: &'static str = "ImGui";
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

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

    let mut ui_manager = ui::Manager::create(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)?;
    let ui_window_1 = ui_manager.add_window("window 1")?;
    let ui_window_2 = ui_manager.add_window("window 2")?;
    let ui_window_3 = ui_manager.add_window("window 3")?;

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
                    // on_cursor_pos_event(&mut window, x, y);
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