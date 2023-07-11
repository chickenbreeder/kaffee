#[derive(Debug)]
pub enum ErrorKind {
    IoError,
    ImageError,
    Gfx(GfxErrorKind),
}

#[derive(Debug)]
pub enum GfxErrorKind {
    SurfaceError,
}

impl From<std::io::Error> for ErrorKind {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}
