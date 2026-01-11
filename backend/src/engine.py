
import io
import os
import torch
import soundfile as sf
import chatterbox

class SynthesisEngine:
    _instance = None
    _model = None

    @classmethod
    def get_instance(cls):
        if cls._instance is None:
            cls._instance = SynthesisEngine()
        return cls._instance

    def __init__(self):
        if self._model is None:
            # Determine device
            device = "cuda" if torch.cuda.is_available() else "cpu"
            print(f"Initializing SynthesisEngine on {device}...")
            
            # Load model
            self._model = chatterbox.ChatterboxTTS.from_pretrained(device)
            print(f"SynthesisEngine initialized. Sample Rate: {self._model.sr}")

    def generate(self, text: str, reference_audio_path: str) -> io.BytesIO:
        """
        Generates audio from text using the reference audio for voice cloning/style.
        Returns a BytesIO object containing the WAV data.
        """
        if not os.path.exists(reference_audio_path):
            raise FileNotFoundError(f"Reference audio not found: {reference_audio_path}")

        # Generate audio
        # audio_prompt_path argument maps to reference_audio_path
        # The model returns a tensor of shape (1, samples)
        with torch.no_grad():
            output_tensor = self._model.generate(text, audio_prompt_path=reference_audio_path)

        # Convert to numpy and squeeze to (samples,)
        audio_data = output_tensor.squeeze().cpu().numpy()

        # Write to WAV in memory
        buffer = io.BytesIO()
        sf.write(buffer, audio_data, self._model.sr, format='WAV')
        buffer.seek(0)
        
        return buffer
