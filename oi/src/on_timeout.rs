use super::norm_path;
use serde_yaml::Value;
use std::path::PathBuf;

#[derive(Debug)]
pub struct OnTimeout {
    pub play: Option<PathBuf>,
}

impl OnTimeout {
    #[inline]
    fn default_play() -> PathBuf {
        PathBuf::from("/usr/share/oi/notification.wav")
    }

    pub fn new(value: &Value) -> Self {
        match value.get("on-timeout") {
            Some(value) => match value {
                Value::Mapping(on_timeout_map) => {
                    match on_timeout_map.get(&Value::String("play".to_owned())) {
                        Some(play_value) => match play_value {
                            Value::String(play_string) => Self {
                                play: Some(norm_path(&PathBuf::from(play_string))),
                            },
                            Value::Null => Self { play: None },
                            Value::Bool(should_play) => {
                                if *should_play {
                                    Self::default()
                                } else {
                                    Self { play: None }
                                }
                            }
                            _ => Self::default(),
                        },
                        None => Self::default(),
                    }
                }
                _ => Self::default(),
            },
            None => Self::default(),
        }
    }
}

impl Default for OnTimeout {
    fn default() -> Self {
        Self {
            play: Some(Self::default_play()),
        }
    }
}
