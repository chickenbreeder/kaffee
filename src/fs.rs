use std::{fs, path::Path};

use crate::error::ErrorKind;

pub(crate) fn load_file<P>(path: P) -> Result<Vec<u8>, ErrorKind>
where
    P: AsRef<Path>,
{
    Ok(fs::read(path)?)
}
