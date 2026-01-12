use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Mp3,
    Ogg,
    Flac,
}

impl fmt::Display for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioFormat::Wav => write!(f, "WAV"),
            AudioFormat::Mp3 => write!(f, "MP3"),
            AudioFormat::Ogg => write!(f, "OGG"),
            AudioFormat::Flac => write!(f, "FLAC"),
        }
    }
}

impl AudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            AudioFormat::Wav => "wav",
            AudioFormat::Mp3 => "mp3",
            AudioFormat::Ogg => "ogg",
            AudioFormat::Flac => "flac",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualitySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioPreviewRequest {
    pub history_id: String,
    pub pitch_shift: f32,
    pub speed_factor: f32,
    pub preview_duration: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioPreviewResponse {
    pub preview_url: String,
    pub duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioProcessRequest {
    pub history_id: String,
    pub pitch_shift: f32,
    pub speed_factor: f32,
    pub output_format: AudioFormat,
    pub quality_settings: QualitySettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioProcessResponse {
    pub processed_file_path: String,
    pub duration: f32,
    pub format: String,
    pub file_size: i64,
    pub history_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioFormatInfo {
    pub value: String,
    pub label: String,
    pub description: String,
}
