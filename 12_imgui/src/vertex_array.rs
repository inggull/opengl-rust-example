pub struct VertexArray {
    vertex_array: u32,
}

impl VertexArray {
    pub fn create() -> VertexArray {
        let mut vertex_array = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
        }
        spdlog::info!("Created vertex array({})", vertex_array);
        
        VertexArray { vertex_array }
    }

    pub fn set(&self, index: u32, size: i32, type_: u32, normalized: u8, stride: i32, offset: *const std::ffi::c_void) {
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(index, size, type_, normalized, stride, offset);
        }
    }

    pub fn get(&self) -> u32 {
        self.vertex_array
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vertex_array);
        }
        spdlog::info!("Dropped vertex array({})", self.vertex_array);
    }
}