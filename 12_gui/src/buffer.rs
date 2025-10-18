pub struct Buffer {
    buffer: u32,
    type_: u32,
    usage: u32,
}

impl Buffer {
    pub fn create(type_: u32, data_size: isize, data: *const std::ffi::c_void, usage: u32) -> Buffer {
        let mut buffer = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer);
            spdlog::info!("Created buffer({})", buffer);
            gl::BindBuffer(type_, buffer);
            gl::BufferData(type_, data_size, data, usage);
        }
        
        Buffer { buffer, type_, usage }
    }

    pub fn get(&self) -> u32 {
        self.buffer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.type_, self.buffer);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.buffer);
        }
        spdlog::info!("Dropped buffer({})", self.buffer);
    }
}