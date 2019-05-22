// for gl-rs (OpenGL 4+)
// pub use gl;

// Custom OpenGL 2.1
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use glfw;
