use std::fs;
use std::path::PathBuf;
use crate::cli::CryptumResult;

pub fn write_data_to_disk(data: String, path: PathBuf) -> CryptumResult<()> {
    fs::write(path, data)?;
    Ok(())
}