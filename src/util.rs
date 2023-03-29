pub fn assemble_convo_path() -> Result<std::path::PathBuf, std::io::Error> {
    if let Some(mut dir_buf) = dirs::home_dir() {
        dir_buf.push(".convos");
        Ok(dir_buf)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Unable to find home dir.",
        ))
    }
}