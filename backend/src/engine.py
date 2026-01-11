
import io
import os
import torch
import soundfile as sf
try:
    from chatterbox import ChatterboxTTS
    from chatterbox.tts_turbo import ChatterboxTurboTTS
except ImportError:
    print("WARNING: chatterbox module not found. Using Mock implementation.")
    import torch
    
    class MockModel:
        def __init__(self, model_name="default"):
            self.sr = 24000
            self.model_name = model_name
            
        def generate(self, text, audio_prompt_path=None, **kwargs):
            print(f"Mock [{self.model_name}] generating audio for: {text}")
            return torch.zeros(1, 48000)

    class MockChatterboxTTS:
        @staticmethod
        def from_pretrained(device):
            return MockModel("standard")

    class MockChatterboxTurboTTS:
        @staticmethod
        def from_pretrained(device):
            return MockModel("turbo")

    class MockChatterbox:
        ChatterboxTTS = MockChatterboxTTS
        ChatterboxTurboTTS = MockChatterboxTurboTTS
        
    chatterbox = MockChatterbox()
    # Mocking the imports if they fail
    ChatterboxTTS = MockChatterboxTTS
    ChatterboxTurboTTS = MockChatterboxTurboTTS

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
            print(f"Initializing SynthesisEngine using Chatterbox-Turbo (350M)...")
            self._model = ChatterboxTurboTTS.from_pretrained(device)
            print(f"SynthesisEngine initialized. Sample Rate: {self._model.sr}")

    def generate(self, text: str, reference_audio_path: str, cfg_weight: float = 0.5, exaggeration: float = 0.5) -> io.BytesIO:
        """
        Generates audio from text using the reference audio for voice cloning/style.
        
        Args:
            text: The text to synthesize
            reference_audio_path: Path to the reference audio file
            cfg_weight: Classifier-free guidance weight (default: 0.5)
            exaggeration: Exaggeration level for expressive speech (default: 0.5)
            
        Returns a BytesIO object containing the WAV data.
        """
        if not os.path.exists(reference_audio_path):
            raise FileNotFoundError(f"Reference audio not found: {reference_audio_path}")

        # Generate audio
        # audio_prompt_path argument maps to reference_audio_path
        # The model returns a tensor of shape (1, samples)
        with torch.no_grad():
            output_tensor = self._model.generate(
                text, 
                audio_prompt_path=reference_audio_path,
                cfg_weight=cfg_weight,
                exaggeration=exaggeration
            )

        # Convert to numpy and squeeze to (samples,)
        audio_data = output_tensor.squeeze().cpu().numpy()

        # Write to WAV in memory
        buffer = io.BytesIO()
        sf.write(buffer, audio_data, self._model.sr, format='WAV')
        buffer.seek(0)
        
        return buffer
