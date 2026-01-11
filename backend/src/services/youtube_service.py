import subprocess
import os
import uuid
import re
import logging
from pathlib import Path
from typing import Optional

# Configure logging
logger = logging.getLogger(__name__)

# Global progress tracker
# task_id -> progress (float 0-100)
_progress_tracker = {}

class YouTubeService:
    @staticmethod
    def get_progress(task_id: str) -> float:
        """Get the current progress for a task."""
        return _progress_tracker.get(task_id, 0.0)

    @staticmethod
    def _parse_progress(line: str) -> Optional[float]:
        """Parse percentage from yt-dlp output line."""
        # Example: [download]  10.5% of 10.00MiB at  1.56MiB/s ETA 00:06
        match = re.search(r'(\d+\.\d+)%', line)
        if match:
            return float(match.group(1))
        return None

    @staticmethod
    def extract_audio(url: str, start_time: str, end_time: str, output_dir: str, task_id: Optional[str] = None) -> str:
        """
        Downloads a section of audio from a YouTube video and saves it as a WAV file.
        Reports progress to the global tracker if task_id is provided.
        """
        logger.info(f"Starting audio extraction from {url} (Task: {task_id})")
        
        if task_id:
            _progress_tracker[task_id] = 0.0

        # Validate inputs
        YouTubeService._validate_youtube_url(url)
        YouTubeService._validate_time_format(start_time, "start time")
        YouTubeService._validate_time_format(end_time, "end time")
        
        start_seconds = YouTubeService._time_to_seconds(start_time)
        end_seconds = YouTubeService._time_to_seconds(end_time)
        if start_seconds >= end_seconds:
            raise ValueError("Start time must be before end time")
        
        os.makedirs(output_dir, exist_ok=True)
        
        file_id = str(uuid.uuid4())[:8]
        filename = f"yt_{file_id}.wav"
        final_path = os.path.join(output_dir, filename)
        
        section_arg = f"*{start_time}-{end_time}"
        
        current_dir = os.path.dirname(os.path.abspath(__file__))
        project_root = os.path.dirname(os.path.dirname(os.path.dirname(current_dir)))
        venv_yt_dlp = os.path.join(project_root, "backend", "venv", "bin", "yt-dlp")
        
        yt_dlp_cmd = venv_yt_dlp if os.path.exists(venv_yt_dlp) else "yt-dlp"
            
        cmd = [
            yt_dlp_cmd,
            "-x",
            "--audio-format", "wav",
            "--download-sections", section_arg,
            "--force-overwrites",
            "--newline", # Ensure progress is printed on new lines
            "--progress", 
            "-o", final_path,
            url
        ]
        
        try:
            logger.info(f"Executing: {' '.join(cmd)}")
            process = subprocess.Popen(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1,
                universal_newlines=True
            )
            
            for line in process.stdout:
                # logger.debug(f"yt-dlp: {line.strip()}")
                progress = YouTubeService._parse_progress(line)
                if progress is not None and task_id:
                    _progress_tracker[task_id] = progress
            
            process.wait(timeout=120)
            
            if process.returncode != 0:
                raise subprocess.CalledProcessError(process.returncode, cmd)

            if not os.path.exists(final_path):
                raise Exception("Audio extraction completed but output file not found.")
            
            if task_id:
                _progress_tracker[task_id] = 100.0
                
            return final_path
            
        except subprocess.TimeoutExpired:
            process.kill()
            raise Exception("Download timed out.")
        except subprocess.CalledProcessError as e:
            raise Exception(f"Download failed with exit code {e.returncode}")
        except Exception as e:
            logger.error(f"Unexpected error during extraction: {str(e)}")
            raise Exception(f"Audio extraction failed: {str(e)}")
        finally:
            if task_id and task_id in _progress_tracker and _progress_tracker[task_id] == 100.0:
                # Keep it for a while? Or let the SSE endpoint clean it up
                pass
