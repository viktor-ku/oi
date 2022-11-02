use super::norm_path;
use serde::Serialize;
use serde_yaml::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct OnTimeout {
    pub play: Option<PathBuf>,
}

impl OnTimeout {
    #[inline]
    pub fn new() -> Self {
        Self { play: None }
    }

    pub fn with(value: &Value) -> Self {
        match value.get("on-timeout") {
            Some(value) => match value {
                Value::Mapping(on_timeout_map) => {
                    match on_timeout_map.get(&Value::String("play".to_owned())) {
                        Some(play_value) => match play_value {
                            Value::String(play_string) => Self {
                                play: Some(norm_path(&PathBuf::from(play_string))),
                            },
                            _ => Self::new(),
                        },
                        None => Self::new(),
                    }
                }
                _ => Self::new(),
            },
            None => Self::new(),
        }
    }
}
