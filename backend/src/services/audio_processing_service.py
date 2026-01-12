import os
import librosa
import soundfile as sf
import numpy as np
from pydub import AudioSegment
from typing import Tuple, Optional
import tempfile
import time
from pathlib import Path
from ..models.audio_processing import AudioFormat, QualitySettings


class AudioProcessingService:
    """Service for audio processing operations including pitch shifting, speed adjustment, and format conversion"""
    
    def __init__(self, preview_dir: str = None):
        """
        Initialize the audio processing service
        
        Args:
            preview_dir: Directory to store preview files (defaults to system temp)
        """
        self.preview_dir = preview_dir or os.path.join(tempfile.gettempdir(), "voicebox_previews")
        os.makedirs(self.preview_dir, exist_ok=True)
    
    def load_audio_file(self, path: str) -> Tuple[np.ndarray, int]:
        """
        Load audio file using librosa
        
        Args:
            path: Path to the audio file
            
        Returns:
            Tuple of (audio data as numpy array, sample rate)
        """
        try:
            audio, sr = librosa.load(path, sr=None, mono=True)
            return audio, sr
        except Exception as e:
            raise ValueError(f"Failed to load audio file: {str(e)}")
    
    def apply_pitch_shift(self, audio: np.ndarray, sr: int, semitones: float) -> np.ndarray:
        """
        Apply pitch shift to audio
        
        Args:
            audio: Audio data as numpy array
            sr: Sample rate
            semitones: Number of semitones to shift (-12 to +12)
            
        Returns:
            Pitch-shifted audio as numpy array
        """
        if semitones == 0:
            return audio
        
        try:
            shifted = librosa.effects.pitch_shift(audio, sr=sr, n_steps=semitones)
            return shifted
        except Exception as e:
            raise ValueError(f"Failed to apply pitch shift: {str(e)}")
    
    def apply_speed_change(self, audio: np.ndarray, sr: int, speed_factor: float) -> np.ndarray:
        """
        Apply speed change to audio (time-stretching, preserves pitch)
        
        Args:
            audio: Audio data as numpy array
            sr: Sample rate
            speed_factor: Speed multiplier (0.5 = half speed, 2.0 = double speed)
            
        Returns:
            Time-stretched audio as numpy array
        """
        if speed_factor == 1.0:
            return audio
        
        try:
            # librosa.effects.time_stretch uses rate parameter (inverse of speed_factor)
            stretched = librosa.effects.time_stretch(audio, rate=speed_factor)
            return stretched
        except Exception as e:
            raise ValueError(f"Failed to apply speed change: {str(e)}")
    
    def numpy_to_audio_segment(self, audio: np.ndarray, sr: int) -> AudioSegment:
        """
        Convert numpy array to pydub AudioSegment
        
        Args:
            audio: Audio data as numpy array
            sr: Sample rate
            
        Returns:
            AudioSegment object
        """
        # Convert to 16-bit PCM
        audio_int16 = (audio * 32767).astype(np.int16)
        
        # Create AudioSegment
        audio_segment = AudioSegment(
            audio_int16.tobytes(),
            frame_rate=sr,
            sample_width=2,  # 16-bit = 2 bytes
            channels=1  # mono
        )
        return audio_segment
    
    def convert_to_format(
        self, 
        audio_segment: AudioSegment, 
        format: AudioFormat, 
        quality: QualitySettings
    ) -> bytes:
        """
        Convert AudioSegment to target format
        
        Args:
            audio_segment: AudioSegment to convert
            format: Target audio format
            quality: Quality settings for the conversion
            
        Returns:
            Audio data as bytes
        """
        # Prepare export parameters
        export_params = {}
        
        if format == AudioFormat.MP3:
            bitrate = quality.bitrate or "192k"
            export_params = {"format": "mp3", "bitrate": bitrate}
        elif format == AudioFormat.OGG:
            quality_val = quality.quality or 5
            export_params = {"format": "ogg", "codec": "libvorbis", "parameters": ["-q:a", str(quality_val)]}
        elif format == AudioFormat.FLAC:
            compression = quality.compression or 5
            export_params = {"format": "flac", "parameters": ["-compression_level", str(compression)]}
        elif format == AudioFormat.WAV:
            export_params = {"format": "wav"}
        
        # Export to bytes
        with tempfile.NamedTemporaryFile(suffix=f".{format.value}", delete=False) as tmp_file:
            tmp_path = tmp_file.name
        
        try:
            audio_segment.export(tmp_path, **export_params)
            with open(tmp_path, "rb") as f:
                audio_bytes = f.read()
            return audio_bytes
        finally:
            if os.path.exists(tmp_path):
                os.remove(tmp_path)
    
    def generate_output_filename(
        self, 
        original_name: str, 
        pitch: float, 
        speed: float, 
        format: str
    ) -> str:
        """
        Generate descriptive filename for processed audio
        
        Args:
            original_name: Original filename (without extension)
            pitch: Pitch shift in semitones
            speed: Speed factor
            format: Output format
            
        Returns:
            Generated filename
        """
        # Remove extension from original name
        base_name = os.path.splitext(original_name)[0]
        
        # Build suffix based on applied effects
        suffix_parts = []
        if pitch != 0:
            suffix_parts.append(f"p{pitch:+.1f}".replace("+", ""))
        if speed != 1.0:
            suffix_parts.append(f"s{speed:.1f}")
        
        suffix = "_" + "_".join(suffix_parts) if suffix_parts else ""
        
        return f"{base_name}{suffix}.{format}"
    
    def cleanup_preview_files(self, max_age_seconds: int = 300):
        """
        Remove preview files older than max_age_seconds
        
        Args:
            max_age_seconds: Maximum age of preview files in seconds (default: 5 minutes)
        """
        current_time = time.time()
        for file_path in Path(self.preview_dir).glob("preview_*.wav"):
            if current_time - file_path.stat().st_mtime > max_age_seconds:
                try:
                    file_path.unlink()
                except Exception:
                    pass  # Ignore errors during cleanup
    
    def generate_preview(
        self,
        audio_path: str,
        pitch_shift: float,
        speed_factor: float,
        duration: float = 10.0
    ) -> str:
        """
        Generate preview file with effects applied
        
        Args:
            audio_path: Path to the original audio file
            pitch_shift: Pitch shift in semitones
            speed_factor: Speed multiplier
            duration: Duration of preview in seconds
            
        Returns:
            Path to the generated preview file
        """
        # Load audio
        audio, sr = self.load_audio_file(audio_path)
        
        # Trim to preview duration
        max_samples = int(duration * sr)
        audio = audio[:max_samples]
        
        # Apply effects
        if pitch_shift != 0:
            audio = self.apply_pitch_shift(audio, sr, pitch_shift)
        if speed_factor != 1.0:
            audio = self.apply_speed_change(audio, sr, speed_factor)
        
        # Save preview file
        preview_filename = f"preview_{int(time.time() * 1000)}.wav"
        preview_path = os.path.join(self.preview_dir, preview_filename)
        
        sf.write(preview_path, audio, sr)
        
        # Cleanup old previews
        self.cleanup_preview_files()
        
        return preview_path
    
    def process_audio_file(
        self,
        audio_path: str,
        output_path: str,
        pitch_shift: float,
        speed_factor: float,
        output_format: AudioFormat,
        quality_settings: QualitySettings
    ) -> dict:
        """
        Process full audio file with effects and save
        
        Args:
            audio_path: Path to the original audio file
            output_path: Path where processed file should be saved
            pitch_shift: Pitch shift in semitones
            speed_factor: Speed multiplier
            output_format: Target audio format
            quality_settings: Quality settings for conversion
            
        Returns:
            Dictionary with metadata (duration, file_size, format)
        """
        # Load audio
        audio, sr = self.load_audio_file(audio_path)
        
        # Apply effects
        if pitch_shift != 0:
            audio = self.apply_pitch_shift(audio, sr, pitch_shift)
        if speed_factor != 1.0:
            audio = self.apply_speed_change(audio, sr, speed_factor)
        
        # Convert to AudioSegment
        audio_segment = self.numpy_to_audio_segment(audio, sr)
        
        # Convert to target format
        audio_bytes = self.convert_to_format(audio_segment, output_format, quality_settings)
        
        # Save to output path
        os.makedirs(os.path.dirname(output_path), exist_ok=True)
        with open(output_path, "wb") as f:
            f.write(audio_bytes)
        
        # Calculate metadata
        duration = len(audio) / sr
        file_size = len(audio_bytes)
        
        return {
            "duration": duration,
            "file_size": file_size,
            "format": output_format.value
        }
