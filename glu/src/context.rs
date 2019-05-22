use super::deps::gl;
use super::deps::glfw;
use super::deps::glfw::Context;

pub fn load_context(window: &mut glfw::Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
}
