mod errors;
mod common;
mod shader;

use glfw::Context;
use glad::gl;

const WINDOW_NAME: &'static str = "Shader";
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
    let (mut window, _glfw_receiver) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_NAME, glfw::WindowMode::Windowed).ok_or(errors::Error::CreateWindowError)?;
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
        gl::ClearColor(0.2, 0.2, 0.2, 1.0); // State-setting function
    }

    let vertex_shader = shader::Shader::create("shader/simple.vert", gl::VERTEX_SHADER)?;
    let fragment_shader = shader::Shader::create("shader/simple.frag", gl::FRAGMENT_SHADER)?;
    spdlog::info!("Created vertex shader({})", vertex_shader.get());
    spdlog::info!("Created fragment shader({})", fragment_shader.get());

    window.set_framebuffer_size_callback(on_frame_buffer_size_event);
    window.set_key_callback(on_key_event);

    // Start main loop
    spdlog::info!("Start main loop");
    while !window.should_close() {
        glfw.poll_events();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT); // State-using function
        }
        window.swap_buffers();
    }

    Ok(())
}

fn on_frame_buffer_size_event(_: &mut glfw::Window, width: i32, height: i32) {
    spdlog::info!("FramebufferSize changed: {width} x {height}");
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