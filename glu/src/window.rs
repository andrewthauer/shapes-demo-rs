use std::sync::mpsc::Receiver;

use super::deps::gl;
use super::deps::glfw;
use super::deps::glfw::Context;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowOptions {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions {
            width: 800,
            height: 600,
            title: String::from("OpenGL Window"),
        }
    }
}

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn create(options: WindowOptions) -> Window {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(
                options.width,
                options.height,
                &options.title[..],
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        return Window {
            glfw,
            window,
            events,
        };
    }

    pub fn load_context(&mut self) {
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
    }

    pub fn event_loop(&mut self, render: &Fn()) {
        while !self.window.should_close() {
            self::process_events(&mut self.window, &self.events);
            render();
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        process_event(window, event);
    }
}

fn process_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        // glfw::WindowEvent::FramebufferSize(width, height) => {
        //     // make sure the viewport matches the new window dimensions; note that width and
        //     // height will be significantly larger than specified on retina displays.
        //     unsafe { gl::Viewport(0, 0, width, height) }
        // }
        glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
