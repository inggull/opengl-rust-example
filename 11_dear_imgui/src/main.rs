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

const WINDOW_NAME: &'static str = "DearImGui";
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
    }

    let mut imgui_context = imgui::Context::create();
    imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
    if !imgui_context.fonts().is_built() {
        spdlog::info!("No fonts build");
    } else {
        spdlog::info!("Fonts build");
    }
    imgui_context.set_ini_filename(None); // Disable saving imgui.ini
    let mut imgui_glfw = imgui_glfw_rs::ImguiGLFW::new(&mut imgui_context, &mut window);

    let mut context = context::Context::create()?;

    // Start main loop
    spdlog::info!("Start main loop");
    let mut current_time;
    let mut last_time= 0f32;
    let mut delta_time;
    while !window.should_close() {
        current_time = glfw.get_time() as f32;
        delta_time = current_time - last_time;
        last_time = current_time;

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
            imgui_glfw.handle_event(&mut imgui_context, &event);
        }

        let ui = imgui_glfw.frame(&mut window, &mut imgui_context);

        context.process_input(&window, delta_time);
        context.render(current_time, ui);

        imgui_glfw.draw(&mut imgui_context, &mut window);

        window.swap_buffers();
        // std::thread::sleep(std::time::Duration::from_millis(1));
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