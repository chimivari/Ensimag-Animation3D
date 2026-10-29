use std::ffi::CStr;

pub enum ShaderSourceType
{
    String(String),
    FileName(String)
}


pub struct ShaderConfig
{
    pub name: String,
    pub source_type: ShaderSourceType
}

/// OpenGL compiled and linked shaders (vertex & fragment)
pub struct ShaderProgram
{
    pub vertex_shader: ShaderConfig,
    pub fragment_shader: ShaderConfig,
    pub gl_program_slot: u32
}

impl ShaderProgram
{
    // TODO: Proper error handling
    pub fn try_new(vertex_shader_config: ShaderConfig, fragment_shader_config: ShaderConfig) -> Result<Self, String>
    {
        let vertex_shader = Shader::from_config(&vertex_shader_config, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::from_config(&fragment_shader_config, gl::FRAGMENT_SHADER)?;
        
        let shader_program: u32;
        let mut success: i32 = 0;

        unsafe
        {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader.gl_index);
            gl::AttachShader(shader_program, fragment_shader.gl_index);
            gl::LinkProgram(shader_program);

            // Check for compiling issue
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success as *mut i32);
            if success == 0
            {
                let mut error_message = [0u8; 1024];
                gl::GetProgramInfoLog(shader_program, 1024, std::ptr::null_mut(), &mut error_message as *mut u8 as *mut i8);

                return Err(
                    str::from_utf8(&error_message)
                    .unwrap_or("Couldn't decode shader program error message")
                    .to_string()
                )
            }
        }

        Ok(Self { 
                vertex_shader: vertex_shader_config,
                fragment_shader: fragment_shader_config,
                gl_program_slot: shader_program
        })
    }
}

impl Drop for ShaderProgram
{
    fn drop(&mut self)
    {
        if self.gl_program_slot == u32::MAX { return; }

        unsafe { gl::DeleteProgram(self.gl_program_slot); }

        println!("DEBUG: Freed shader program {}", self.gl_program_slot);
        self.gl_program_slot = u32::MAX;

    }
}

impl ShaderConfig
{
    // TODO: Return proper Error
    // Not very clean or memory efficient or whatever
    pub fn get_source_code(&self) -> Result<String, ()>
    {
        match &self.source_type {
            ShaderSourceType::String(s) => Ok(s.to_string()),
            ShaderSourceType::FileName(f) =>
            {
                match std::fs::read_to_string(f) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(())
                }
            }
        }
    }
}

pub struct Shader
{
    pub gl_index: u32
}

impl Drop for Shader
{
    fn drop(&mut self)
    {
        if self.gl_index == u32::MAX { return; }

        unsafe { gl::DeleteShader(self.gl_index); }

        println!("DEBUG: Freed shader {}", self.gl_index);
        self.gl_index = u32::MAX;
    }

}

impl Shader
{
    // TODO: Proper error handling
    pub fn from_config(shader_config: &ShaderConfig, gl_shader_type: u32) -> Result<Self, String>
    {
        let mut source = match shader_config.get_source_code()
        {
            Ok(s) => s,
            Err(_) => return Err("Couldn't get source code for shader".to_string())
        };

        source += "\0";
        let gl_index = Shader::compile_shader(source.as_ptr() as *const i8, gl_shader_type)?;

        Ok(Self
        {
            gl_index
        })
    }

    /// Returns the corresponding OpenGL slot for the compiled shader
    /// /!\ source_str_ptr NEEDS to point to a null-terminated sequence of characters
    fn compile_shader(source_str_ptr: *const i8, shader_type: u32 ) -> Result<u32, String>
    {
        let shader: u32;
        unsafe
        {
            println!("{:?}", CStr::from_ptr(source_str_ptr));
            shader = gl::CreateShader(shader_type);

            gl::ShaderSource(
                shader,
                1,
                // Pointer conversion shenanigans for C library calls
                &source_str_ptr as *const *const i8,
                std::ptr::null());

            gl::CompileShader(shader);

            let mut success: i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            // success == 0 is failed compilation
            if success != 0 { return Ok(shader); }


            // In case of failure
            let mut error_message = [0i8; 1024];
            gl::GetShaderInfoLog(
                shader,
                1024 - 1,
                std::ptr::null_mut(),
                // Once again conversion hell, converting u8 to i8 for 
                //     C's signed "byte" or "char" array
                &mut error_message as *mut i8);


            return Err(CStr::from_ptr(&error_message as *const i8).to_str().to_owned().unwrap_or("Couldn't decode returned OpenGL shader compilation error message from c_string").to_string());
        }
    }



}
