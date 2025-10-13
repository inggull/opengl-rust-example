mod errors;
mod common;
mod shader;
mod program;
mod context;
mod vertex_array;
mod buffer;
mod texture;
mod image;

use glfw::Context;
use glad::gl;

const WINDOW_NAME: &'static str = "Interactive Camera";
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
    window.set_key_polling(true); // window.set_key_callback(on_key_event);
    window.set_framebuffer_size_polling(true); // window.set_framebuffer_size_callback(on_frame_buffer_size_event);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    // Initialize glad
    spdlog::info!("Initialize glad");
    gl::load(|procname| {
        match window.get_proc_address(procname) {
            Some(f) => f as *const _,
            None => {
                spdlog::error!("Failed to initialize glad");
                panic!();
            }
        }
    });

    unsafe {
        let gl_version = gl::GetString(gl::VERSION);
        spdlog::info!("Loaded OpenGL {}", common::c_str_to_string(gl_version.cast()).unwrap_or(String::from("Unknown")));
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32); // State-setting function
    }

    let mut context = context::Context::create()?;

    // Start main loop
    spdlog::info!("Start main loop");
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => { 
                    context.reshape(width as u32, height as u32);
                    on_frame_buffer_size_event(&mut window, width, height);
                }
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => on_key_event(&mut window, key, scancode, action, modifiers),
                glfw::WindowEvent::CursorPos(x, y) => {
                    context.mouse_move(x, y);
                    on_cursur_pos_event(&mut window, x, y);
                }
                glfw::WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                    context.mouse_button(mouse_button, action);
                    on_mouse_button_event(&mut window, mouse_button, action, modifiers);
                }
                _ => {},
            }
        }
        context.process_input(&window);
        context.render(glfw.get_time() as f32);
        window.swap_buffers();
    }

    Ok(())
}

fn on_frame_buffer_size_event(_: &mut glfw::Window, width: i32, height: i32) {
    spdlog::info!("FramebufferSize changed: {} x {}", width, height);
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
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

fn on_cursur_pos_event(_: &mut glfw::Window, x: f64, y: f64) {
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