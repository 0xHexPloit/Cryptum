use std::fs;
use std::path::PathBuf;
use crate::CryptumResult;
use crate::errors::CryptumError;

pub fn write_data_to_disk(data: String, path: PathBuf) -> CryptumResult<()> {
    fs::write(path.clone(), data).map_err(|err| CryptumError::IO(
        format!("Error while writing to `{}`: {}",
                path.display(),
                err)
    ))?;
    Ok(())
}

pub fn read_data_from_file(path: PathBuf) -> CryptumResult<String> {
    let data = fs::read_to_string(path.clone()).map_err(|err| CryptumError::IO(
        format!("An error occurred while trying to read `{}`: {}",
                path.display(),
                err)
    ))?;
    Ok(data)
}