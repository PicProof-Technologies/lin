use std::fmt::Debug;

pub enum ImageFormat {
    JPEG,
    PNG,
}
impl Default for ImageFormat {
    fn default() -> Self {
        Self::PNG
    }
}
impl Clone for ImageFormat {
    fn clone(&self) -> Self {
        match *self {
            ImageFormat::JPEG => ImageFormat::JPEG,
            ImageFormat::PNG => ImageFormat::PNG,
        }
    }
}
impl Debug for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ImageFormat::JPEG => write!(f, "jpeg"),
            ImageFormat::PNG => write!(f, "png"),
        }
    }
}
impl From<ImageFormat> for String {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::JPEG => "jpeg".to_string(),
            ImageFormat::PNG => "png".to_string(),
        }
    }
}
impl TryFrom<String> for ImageFormat {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "jpeg" => Ok(ImageFormat::JPEG),
            "png" => Ok(ImageFormat::PNG),
            _ => Err("The format of image is not supposed".to_string()),
        }
    }
}
