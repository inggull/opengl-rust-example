use super::image;

pub struct Texture {
    texture: u32,
}

impl Texture {
    pub fn create() -> Texture {
        let mut texture = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            spdlog::info!("Created texture({})", texture);
            // bind and set default filter and wrap option
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE.cast_signed());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE.cast_signed());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR.cast_signed());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR.cast_signed());
        }

        Texture { texture }
    }

    pub fn get(&self) -> u32 {
        self.texture
    }

    pub fn bind(&self) -> &Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
        self
    }

    pub fn set_filter(&self, min_filter: u32, mag_filter: u32) -> &Self {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter.cast_signed());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter.cast_signed());
        }
        self
    }

    pub fn set_wrap(&self, wrap_s: u32, wrap_t: u32) -> &Self {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s.cast_signed());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t.cast_signed());
        }
        self
    }

    pub fn set_texture(&self, image: &image::Image) -> &Self {
        self.bind();
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA.cast_signed(), image.get_width().cast_signed(), image.get_height().cast_signed(), 0,
                match image.get_channel_count() {
                    1 => gl::RED,
                    2 => gl::RG,
                    3 => gl::RGB,
                    _ => gl::RGBA,
                },
                gl::UNSIGNED_BYTE, image.get_data().as_ptr().cast()
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { 
            gl::DeleteTextures(1, &mut self.texture);
        }
        spdlog::info!("Dropped texture({})", self.texture);
    }
}