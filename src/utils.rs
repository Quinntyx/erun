use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use icns::IconFamily;

#[allow(unreachable_code, unused_variables)]
pub fn load_icon_to_uri(path: PathBuf) -> Result<String, anyhow::Error> {
    return Ok("".into());
    if path.extension().is_some_and(|x| x == "icns") {
        let family = IconFamily::read(BufReader::new(File::open(path)?))?;
        let icon = family.get_icon_with_type(family.available_icons()[0])?;
        Ok(format!(
            "bytes://{}",
            icon.into_data()
                .iter()
                .map(|i| i.to_string())
                .collect::<String>()
        ))
    } else {
        Ok(format!(
            "file://{}",
            path.into_os_string()
                .into_string()
                .expect("Path should be valid")
        ))
    }
}
