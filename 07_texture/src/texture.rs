use glad::gl;

pub struct Texture {
    texture: u32,
}

impl Texture {
    pub fn create(image: &image::DynamicImage) -> Texture {
        let mut texture = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            spdlog::info!("Created texture({})", texture);
            // bind and set default filter and wrap option
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }
        set_wrap(gl::CLAMP_TO_EDGE, gl::CLAMP_TO_EDGE);
        set_filter(gl::LINEAR_MIPMAP_LINEAR, gl::LINEAR);
        set_texture(image);

        Texture { texture }
    }

    pub fn get(&self) -> u32 {
        self.texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}

pub fn set_filter(min_filter: u32, mag_filter: u32) {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter.cast_signed());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter.cast_signed());
    }
}

pub fn set_wrap(wrap_s: u32, wrap_t: u32) {
    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s.cast_signed());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t.cast_signed());
    }
}

pub fn set_texture(image: &image::DynamicImage) {
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA.cast_signed(), image.width().cast_signed(), image.height().cast_signed(), 0,
            match image.color() {
                image::ColorType::Rgb8 => gl::RGB,
                _ => gl::RGBA,
            },
            match image.color().bits_per_pixel() {
                16 => gl::UNSIGNED_SHORT,
                _ => gl::UNSIGNED_BYTE,
            },
            image.as_bytes().as_ptr().cast()
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { 
            gl::DeleteBuffers(1, &mut self.texture);
        }
        spdlog::info!("Dropped texture({})", self.texture);
    }
}