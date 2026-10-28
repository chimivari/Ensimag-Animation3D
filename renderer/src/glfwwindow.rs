use std::ffi::c_void;
use glfw::{fail_on_errors, Context, Glfw, WindowEvent};

fn framebuffer_size_callback<'a> (_window: &'a mut glfw::Window, width: i32, height: i32)
{
    unsafe { gl::Viewport(0, 0, width, height); }
}

pub struct GLFWWindow
{
    pub glfw: Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl GLFWWindow
{
    // TODO: Return actual errors
    pub fn try_new(width: u32, height: u32, title: &str) -> Result<Self, ()>
    {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(
                width, height, title,
                glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);

        window.set_framebuffer_size_callback(framebuffer_size_callback);
        
        gl::load_with(|s| glfw.get_proc_address_raw(s).expect("GLFW failed to return glfw process address")  as *const c_void);

        gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s).expect("GLFW failed to return viewport proc address") as *const c_void);

        Ok(Self {
            glfw,
            window,
            events
        })

    }

}

