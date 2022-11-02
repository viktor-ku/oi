use anyhow::Result;
use directories::ProjectDirs;
use serde::Serialize;
use serde_yaml::Value;
use std::fs::File;
use std::path::PathBuf;

mod on_timeout;
use on_timeout::OnTimeout;

mod norm_path;
use norm_path::norm_path;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub port: u32,
    pub volume: f32,
    pub on_timeout: OnTimeout,
}

impl Config {
    const DEFAULT_VOLUME: f32 = 0.8;
    const DEFAULT_PORT: u32 = 9191;

    /// Attempts to compute the base config dir for the user,
    /// considering different alternatives for ~/.config accross
    /// different OS
    #[inline]
    pub fn config_dir() -> Option<PathBuf> {
        ProjectDirs::from("com", "oi", "oi").map(|dirs| dirs.config_dir().to_path_buf())
    }

    #[inline]
    fn find_full_config_path() -> Option<PathBuf> {
        Self::config_dir()
            .map(|dir| {
                ["oi.yml", "oi.yaml"]
                    .iter()
                    .map(|filename| dir.join(filename))
                    .find(|path| path.is_file())
                    .clone()
            })
            .flatten()
    }

    fn parse_volume(value: &Value) -> f32 {
        match value.get("volume") {
            Some(num) => match num {
                Value::Number(n) => match n.as_f64() {
                    Some(n) => n as f32,
                    None => Self::DEFAULT_VOLUME,
                },
                _ => Self::DEFAULT_VOLUME,
            },
            None => Self::DEFAULT_VOLUME,
        }
    }

    fn parse_port(value: &Value) -> u32 {
        match value.get("port") {
            Some(val) => match val {
                Value::Number(n) => match n.as_u64() {
                    Some(n) => n as u32,
                    None => Self::DEFAULT_PORT,
                },
                Value::String(val) => match val.parse() {
                    Ok(val) => val,
                    Err(_) => Self::DEFAULT_PORT,
                },
                _ => Self::DEFAULT_PORT,
            },
            None => Self::DEFAULT_PORT,
        }
    }

    pub fn new() -> Result<Self> {
        match Self::find_full_config_path() {
            Some(path) => {
                let rd = File::open(path)?;
                let root: serde_yaml::Value = serde_yaml::from_reader(rd)?;

                Ok(Self {
                    on_timeout: OnTimeout::with(&root),
                    volume: Self::parse_volume(&root),
                    port: Self::parse_port(&root),
                })
            }
            None => Ok(Self::default()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: Self::DEFAULT_VOLUME,
            on_timeout: OnTimeout::new(),
            port: Self::DEFAULT_PORT,
        }
    }
}
