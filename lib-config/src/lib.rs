use directories::ProjectDirs;
use serde_yaml::Value;
use std::fs::File;
use std::path::PathBuf;

mod on_timeout;
use on_timeout::OnTimeout;

mod norm_path;
use norm_path::norm_path;

#[derive(Debug)]
pub struct Config {
    pub volume: f64,
    pub on_timeout: OnTimeout,
}

impl Config {
    pub const DEFAULT_VOLUME: f64 = 0.8;

    #[inline]
    pub fn volume(&self) -> f32 {
        self.volume as f32
    }

    pub fn config_dir() -> Option<PathBuf> {
        match ProjectDirs::from("com", "oi", "oi") {
            Some(dirs) => Some(dirs.config_dir().to_path_buf()),
            None => None,
        }
    }

    fn full_config_path() -> Option<PathBuf> {
        match Self::config_dir() {
            Some(dir) => {
                let config_path = &["oi.yml", "oi.yaml", "oirc", ".oirc"]
                    .iter()
                    .map(|filename| dir.join(filename))
                    .find(|path| path.is_file());

                match config_path {
                    Some(config_path) => Some(config_path.clone()),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn parse_volume(value: &Value) -> f64 {
        let num = value.get("volume").unwrap();
        match num {
            serde_yaml::Value::Number(n) => n.as_f64().unwrap_or(Self::DEFAULT_VOLUME),
            _ => Self::DEFAULT_VOLUME,
        }
    }

    pub fn new() -> Self {
        match Self::full_config_path() {
            Some(path) => {
                let rd = File::open(path).unwrap();
                let value: serde_yaml::Value = serde_yaml::from_reader(rd).unwrap();

                let volume = Self::parse_volume(&value);
                let on_timeout = OnTimeout::new(&value);

                Self { volume, on_timeout }
            }
            None => Self::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: Self::DEFAULT_VOLUME,
            on_timeout: OnTimeout::default(),
        }
    }
}
