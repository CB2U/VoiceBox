import subprocess
import os
import uuid
import re
import logging
from pathlib import Path

# Configure logging
logger = logging.getLogger(__name__)

class YouTubeService:
    @staticmethod
    def _validate_youtube_url(url: str) -> None:
        """
        Validates that the URL is a valid YouTube URL.
        
        Args:
            url: URL to validate
            
        Raises:
            ValueError: If URL is not a valid YouTube URL
        """
        youtube_patterns = [
            r'^https?://(www\.)?youtube\.com/watch\?v=[\w-]+',
            r'^https?://(www\.)?youtu\.be/[\w-]+',
            r'^https?://(www\.)?youtube\.com/shorts/[\w-]+'
        ]
        
        if not any(re.match(pattern, url) for pattern in youtube_patterns):
            raise ValueError("Invalid YouTube URL. Please provide a valid YouTube video URL (e.g., https://www.youtube.com/watch?v=...)")
    
    @staticmethod
    def _validate_time_format(time_str: str, field_name: str) -> None:
        """
        Validates time format (supports HH:MM:SS, MM:SS, or seconds).
        
        Args:
            time_str: Time string to validate
            field_name: Name of the field (for error messages)
            
        Raises:
            ValueError: If time format is invalid
        """
        # Accept formats: HH:MM:SS, MM:SS, or just seconds
        time_patterns = [
            r'^\d+$',  # Just seconds
            r'^\d{1,2}:\d{2}$',  # MM:SS
            r'^\d{1,2}:\d{2}:\d{2}$'  # HH:MM:SS
        ]
        
        if not any(re.match(pattern, time_str.strip()) for pattern in time_patterns):
            raise ValueError(f"Invalid {field_name} format. Use HH:MM:SS, MM:SS, or seconds (e.g., '00:01:30', '1:30', or '90')")
    
    @staticmethod
    def _time_to_seconds(time_str: str) -> int:
        """Convert time string to seconds."""
        parts = time_str.strip().split(':')
        if len(parts) == 1:
            return int(parts[0])
        elif len(parts) == 2:
            return int(parts[0]) * 60 + int(parts[1])
        else:  # HH:MM:SS
            return int(parts[0]) * 3600 + int(parts[1]) * 60 + int(parts[2])
    
    @staticmethod
    def extract_audio(url: str, start_time: str, end_time: str, output_dir: str) -> str:
        """
        Downloads a section of audio from a YouTube video and saves it as a WAV file.
        
        Args:
            url: YouTube video URL
            start_time: Start time (e.g. "00:00:10", "1:30", or "90")
            end_time: End time (e.g. "00:00:20", "2:00", or "120")
            output_dir: Directory to save the file
            
        Returns:
            Path to the saved WAV file
            
        Raises:
            ValueError: If validation fails
            Exception: If download or processing fails
        """
        logger.info(f"Starting audio extraction from {url}")
        
        # Validate inputs
        try:
            YouTubeService._validate_youtube_url(url)
            logger.info("URL validation passed")
        except ValueError as e:
            logger.error(f"URL validation failed: {e}")
            raise
        
        try:
            YouTubeService._validate_time_format(start_time, "start time")
            YouTubeService._validate_time_format(end_time, "end time")
            logger.info("Time format validation passed")
        except ValueError as e:
            logger.error(f"Time format validation failed: {e}")
            raise
        
        # Validate time range
        try:
            start_seconds = YouTubeService._time_to_seconds(start_time)
            end_seconds = YouTubeService._time_to_seconds(end_time)
            
            if start_seconds >= end_seconds:
                raise ValueError("Start time must be before end time")
            
            if start_seconds < 0 or end_seconds < 0:
                raise ValueError("Time values must be positive")
                
            logger.info(f"Time range validation passed: {start_seconds}s to {end_seconds}s")
        except ValueError as e:
            logger.error(f"Time range validation failed: {e}")
            raise
        
        # Create output directory if it doesn't exist
        try:
            os.makedirs(output_dir, exist_ok=True)
            logger.info(f"Output directory ready: {output_dir}")
        except Exception as e:
            logger.error(f"Failed to create output directory: {e}")
            raise Exception(f"Failed to create output directory: {str(e)}")
        
        # Generate unique filename
        file_id = str(uuid.uuid4())[:8]
        filename = f"yt_{file_id}.wav"
        final_path = os.path.join(output_dir, filename)
        logger.info(f"Target output file: {final_path}")
        
        # yt-dlp expects download sections in format *start-end
        section_arg = f"*{start_time}-{end_time}"
        
        # Resolve yt-dlp path
        current_dir = os.path.dirname(os.path.abspath(__file__))
        project_root = os.path.dirname(os.path.dirname(os.path.dirname(current_dir)))
        venv_yt_dlp = os.path.join(project_root, "backend", "venv", "bin", "yt-dlp")
        
        yt_dlp_cmd = "yt-dlp"
        if os.path.exists(venv_yt_dlp):
            yt_dlp_cmd = venv_yt_dlp
            logger.info(f"Using venv yt-dlp: {yt_dlp_cmd}")
        else:
            logger.info("Using system yt-dlp")
            
        cmd = [
            yt_dlp_cmd,
            "-x",
            "--audio-format", "wav",
            "--download-sections", section_arg,
            "--force-overwrites",
            "-o", final_path,
            url
        ]
        
        try:
            logger.info(f"Executing: {' '.join(cmd)}")
            result = subprocess.run(
                cmd, 
                check=True, 
                stdout=subprocess.PIPE, 
                stderr=subprocess.PIPE,
                timeout=120  # 2 minute timeout
            )
            
            logger.info("yt-dlp command completed successfully")
            
            # Verify output file exists
            if not os.path.exists(final_path):
                logger.error(f"Output file not found at {final_path}")
                raise Exception(f"Audio extraction completed but output file not found. This may indicate a yt-dlp processing error.")
            
            file_size = os.path.getsize(final_path)
            logger.info(f"Audio file created successfully: {final_path} ({file_size} bytes)")
            return final_path
            
        except subprocess.TimeoutExpired:
            logger.error("yt-dlp command timed out")
            raise Exception("Download timed out. The video may be too long or your connection is slow. Please try a shorter clip.")
        except subprocess.CalledProcessError as e:
            error_output = e.stderr.decode() if e.stderr else ""
            logger.error(f"yt-dlp failed with exit code {e.returncode}: {error_output}")
            
            # Parse common errors and provide helpful messages
            if "Video unavailable" in error_output or "Private video" in error_output:
                raise Exception("This video is unavailable or private. Please check the URL and try again.")
            elif "Unable to extract" in error_output or "Unsupported URL" in error_output:
                raise Exception("Unable to download from this URL. Please verify it's a valid YouTube video link.")
            elif "ERROR:" in error_output:
                # Extract the actual error message
                error_lines = [line for line in error_output.split('\n') if 'ERROR:' in line]
                if error_lines:
                    raise Exception(f"Download failed: {error_lines[0]}")
            
            raise Exception(f"Download failed. Please check the URL and try again. Details: {error_output[:200]}")
        except FileNotFoundError:
            logger.error("yt-dlp command not found")
            raise Exception("yt-dlp is not installed. Please install it using: pip install yt-dlp")
        except Exception as e:
            logger.error(f"Unexpected error during extraction: {str(e)}")
            raise Exception(f"Audio extraction failed: {str(e)}")
