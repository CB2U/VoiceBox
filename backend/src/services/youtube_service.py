import subprocess
import os
import uuid
from pathlib import Path

class YouTubeService:
    @staticmethod
    def extract_audio(url: str, start_time: str, end_time: str, output_dir: str) -> str:
        """
        Downloads a section of audio from a YouTube video and saves it as a WAV file.
        
        Args:
            url: YouTube video URL
            start_time: Start time (e.g. "00:00:10" or "10")
            end_time: End time (e.g. "00:00:20" or "20")
            output_dir: Directory to save the file
            
        Returns:
            Path to the saved WAV file
        """
        # Create output directory if it doesn't exist
        os.makedirs(output_dir, exist_ok=True)
        
        # Generate unique filename
        file_id = str(uuid.uuid4())[:8]
        filename = f"yt_{file_id}.wav"
        final_path = os.path.join(output_dir, filename)
        
        # yt-dlp expects download sections in format *start-end
        # Example: *10-20 or *00:00:10-00:00:20
        section_arg = f"*{start_time}-{end_time}"
        
        # Build command:
        # -x: Extract audio
        # --audio-format wav: Convert to WAV
        # --download-sections: Download only the specified range
        # -o: Output template
        # --force-overwrites: Overwrite if exists
        
        # Resolve yt-dlp path
        # Try to find it in the venv relative to this file
        # File: backend/src/services/youtube_service.py
        # Venv: backend/venv/bin/yt-dlp
        
        current_dir = os.path.dirname(os.path.abspath(__file__))
        project_root = os.path.dirname(os.path.dirname(os.path.dirname(current_dir))) # .../backend
        venv_yt_dlp = os.path.join(project_root, "backend", "venv", "bin", "yt-dlp")
        
        yt_dlp_cmd = "yt-dlp" # Default to PATH
        if os.path.exists(venv_yt_dlp):
            yt_dlp_cmd = venv_yt_dlp
            
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
            print(f"Running command: {' '.join(cmd)}")
            result = subprocess.run(
                cmd, 
                check=True, 
                stdout=subprocess.PIPE, 
                stderr=subprocess.PIPE
            )
            
            # Post-processing: yt-dlp might append .wav to the filename automatically if not handled carefully
            # with -o, it generally respects it, but if it converts, it might replace ext.
            # Since we requested wav and output is .wav, it should be fine.
            # However, strictly checking what file was created is safer.
            
            if not os.path.exists(final_path):
                # Sometimes yt-dlp might name it differently if it can't convert perfectly or logic differs
                # But with explicit -o and --audio-format, it usually works.
                raise Exception(f"Expected output file not found at {final_path}")
                
            return final_path
            
        except subprocess.CalledProcessError as e:
            error_msg = e.stderr.decode()
            print(f"yt-dlp error: {error_msg}")
            raise Exception(f"Download failed: {error_msg}")
        except Exception as e:
            print(f"General error: {str(e)}")
            raise Exception(f"Extraction failed: {str(e)}")
