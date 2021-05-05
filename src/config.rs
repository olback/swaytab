use {
    crate::{STError, STResult},
    serde::Deserialize,
    std::{fs, path::PathBuf},
    toml,
};

const DEFAULT_CONFIG: &'static str = include_str!("../Swaytab.toml");

#[derive(Debug, Deserialize)]
pub struct STConfig {
    pub command: String,
}

impl STConfig {
    pub fn write_default() -> STResult<()> {
        todo!()
    }

    pub fn load() -> STResult<Self> {
        let raw_str = fs::read_to_string(Self::path()?)?;
        let conf: Self = toml::from_str(&raw_str)?;
        Ok(conf)
    }

    fn path() -> STResult<PathBuf> {
        Ok(dirs::config_dir()
            .ok_or(STError::Other("Failed to find root config dir".into()))?
            .join("swaytab")
            .join("Swaytab.toml"))
    }
}
