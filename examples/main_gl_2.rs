use gl;
use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

fn main_gl_2() {
    // Initialize instance of GLFW (creates windows)
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a new window
    let (window, events) = glfw
        .create_window(
            800,
            600,
            String::from("Hello OpenGL!"),
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Load OpenGl context
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Setup window
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Event loop
    while !window.should_close() {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        // Draw scene background
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw rectangle (using OpengGL 2 API)
        gl::Color3d(1.0, 0.0, 0.0);
        gl::Rectd(-0.5, -0.5, 0.5, 0.5);

        // Finish drawing scene
        gl::Flush();

        // Swap buffers and poll events
        window.swap_buffers();
        glfw.poll_events();
    }
}
