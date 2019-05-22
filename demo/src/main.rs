use glu::gl;

fn main() {
    let mut window = glu::Window::create(glu::WindowOptions {
        title: String::from("Hello Shapes"),
        ..glu::WindowOptions::default()
    });

    window.load_context();
    window.event_loop(&render_scene);
}

pub fn draw_shape() {
    unsafe {
        // gl::Begin(gl::LINES);
        // gl::Color3d(1.0, 0.0, 0.0);
        // gl::Vertex2d(0.0, 0.0);
        // gl::Vertex2d(42.0, 42.0);
        // gl::End();

        gl::Color3d(1.0, 0.0, 0.0);
        gl::Rectd(-0.5, -0.5, 0.5, 0.5);
    }
}

fn render_scene() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        draw_shape();
        gl::Flush();
    }
}
