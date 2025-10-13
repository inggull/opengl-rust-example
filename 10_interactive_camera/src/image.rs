use image::EncodableLayout;

use super::errors;

pub struct Image {
    width: u32,
    height: u32,
    channel_count: u8,
    data: Vec<u8>,
}

impl Image {
    pub fn load<S>(file_path: S) -> Result<Image, errors::Error> where S: AsRef<str> {
        // 이미지 파일과 OpenGL은 이미지의 시작점이 상하 대칭 관계에 있기 때문에 불러온 이미지 파일을 상하 반전시켜야 한다
        let image = image::open(file_path.as_ref())?.flipv();
        let width = image.width();
        let height = image.height();
        let channel_count = image.color().channel_count();
        let data = image.as_bytes().to_owned();

        Ok(Image { width, height, channel_count, data })
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_channel_count(&self) -> u8 {
        self.channel_count
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_bytes()
    }
}