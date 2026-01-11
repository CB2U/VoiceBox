use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::fs::File;

/// Combines multiple WAV files into a single output WAV file.
/// All input files must have the same sample rate, channels, and bits per sample.
pub fn combine_wavs(input_paths: Vec<PathBuf>, output_path: PathBuf) -> Result<(), String> {
    if input_paths.is_empty() {
        return Err("No input files provided".to_string());
    }
    
    // Read the first file to get the audio format
    let first_path = &input_paths[0];
    let first_reader = hound::WavReader::open(first_path)
        .map_err(|e| format!("Failed to open {}: {}", first_path.display(), e))?;
    
    let spec = first_reader.spec();
    
    // Validate all files have the same format
    for path in &input_paths[1..] {
        let reader = hound::WavReader::open(path)
            .map_err(|e| format!("Failed to open {}: {}", path.display(), e))?;
        
        if reader.spec() != spec {
            return Err(format!(
                "File {} has different audio format than the first file",
                path.display()
            ));
        }
    }
    
    // Create the output writer
    let mut writer = hound::WavWriter::create(&output_path, spec)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    
    // Read and write samples from each file
    for path in input_paths {
        let mut reader = hound::WavReader::open(&path)
            .map_err(|e| format!("Failed to open {}: {}", path.display(), e))?;
        
        // Copy samples based on the sample format
        match spec.sample_format {
            hound::SampleFormat::Float => {
                for sample in reader.samples::<f32>() {
                    let sample = sample.map_err(|e| format!("Failed to read sample: {}", e))?;
                    writer.write_sample(sample)
                        .map_err(|e| format!("Failed to write sample: {}", e))?;
                }
            }
            hound::SampleFormat::Int => {
                match spec.bits_per_sample {
                    16 => {
                        for sample in reader.samples::<i16>() {
                            let sample = sample.map_err(|e| format!("Failed to read sample: {}", e))?;
                            writer.write_sample(sample)
                                .map_err(|e| format!("Failed to write sample: {}", e))?;
                        }
                    }
                    32 => {
                        for sample in reader.samples::<i32>() {
                            let sample = sample.map_err(|e| format!("Failed to read sample: {}", e))?;
                            writer.write_sample(sample)
                                .map_err(|e| format!("Failed to write sample: {}", e))?;
                        }
                    }
                    _ => {
                        return Err(format!("Unsupported bits per sample: {}", spec.bits_per_sample));
                    }
                }
            }
        }
    }
    
    writer.finalize()
        .map_err(|e| format!("Failed to finalize output file: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    
    #[test]
    fn test_combine_empty_list() {
        let output = PathBuf::from("test_output.wav");
        let result = combine_wavs(vec![], output);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No input files provided");
    }
    
    // Additional tests would require creating actual WAV files
    // For now, we'll rely on manual testing with real synthesis output
}
