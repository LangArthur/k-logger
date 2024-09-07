use std::{fs::File, io::Read, path::Path};

pub fn get_file_content<P>(path: P) -> Result<String, String>
where
    P: AsRef<Path>,
{
    let mut device_file_content = vec![];
    let mut fd = File::open(path).map_err(|err| err.to_string())?;
    fd.read_to_end(&mut device_file_content)
        .map_err(|err| err.to_string())?;
    String::from_utf8(device_file_content).map_err(|err| err.to_string())
}
