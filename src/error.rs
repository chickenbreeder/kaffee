#[derive(Debug)]
pub enum ErrorKind {
    IoError,
    ImageError,
}

impl From<std::io::Error> for ErrorKind {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<image::ImageError> for ErrorKind {
    fn from(_: image::ImageError) -> Self {
        Self::ImageError
    }
}
