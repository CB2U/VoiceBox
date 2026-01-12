from pydantic import BaseModel, Field
from typing import Optional
from enum import Enum


class AudioFormat(str, Enum):
    WAV = "wav"
    MP3 = "mp3"
    OGG = "ogg"
    FLAC = "flac"


class QualitySettings(BaseModel):
    """Quality settings for audio format conversion"""
    bitrate: Optional[str] = Field(None, description="Bitrate for MP3 (e.g., '192k', '320k')")
    quality: Optional[int] = Field(None, ge=0, le=10, description="Quality for OGG (0-10)")
    compression: Optional[int] = Field(None, ge=0, le=8, description="Compression level for FLAC (0-8)")


class AudioPreviewRequest(BaseModel):
    """Request to generate a preview of audio with effects applied"""
    history_id: str = Field(..., description="ID of the history entry to process")
    pitch_shift: float = Field(0.0, ge=-12.0, le=12.0, description="Pitch shift in semitones (-12 to +12)")
    speed_factor: float = Field(1.0, ge=0.5, le=2.0, description="Speed multiplier (0.5x to 2.0x)")
    preview_duration: float = Field(10.0, gt=0, description="Duration of preview in seconds")


class AudioPreviewResponse(BaseModel):
    """Response containing preview file information"""
    preview_url: str = Field(..., description="URL to access the preview file")
    duration: float = Field(..., description="Duration of the preview in seconds")


class AudioProcessRequest(BaseModel):
    """Request to process full audio file with effects"""
    history_id: str = Field(..., description="ID of the history entry to process")
    pitch_shift: float = Field(0.0, ge=-12.0, le=12.0, description="Pitch shift in semitones (-12 to +12)")
    speed_factor: float = Field(1.0, ge=0.5, le=2.0, description="Speed multiplier (0.5x to 2.0x)")
    output_format: AudioFormat = Field(AudioFormat.WAV, description="Output audio format")
    quality_settings: QualitySettings = Field(default_factory=QualitySettings, description="Quality settings for lossy formats")


class AudioProcessResponse(BaseModel):
    """Response containing processed file information"""
    processed_file_path: str = Field(..., description="Path to the processed file")
    duration: float = Field(..., description="Duration of the processed file in seconds")
    format: str = Field(..., description="Format of the processed file")
    file_size: int = Field(..., description="Size of the processed file in bytes")
    history_id: str = Field(..., description="ID of the new history entry")
