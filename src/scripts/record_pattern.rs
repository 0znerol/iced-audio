use hound::{WavSpec, WavWriter};
use rodio::{Decoder, Source};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn record_pattern(
    beat_pattern: &Vec<Vec<bool>>,
    audio_files: &Vec<String>,
    bpm: u32,
    sequence_length: u32,
    selected_samples: &BTreeMap<usize, String>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path = Path::new("recorded_patterns").join(output_file);
    let mut writer = WavWriter::create(path, spec)?;

    let beat_duration = (60.0 / bpm as f32 * spec.sample_rate as f32) as usize / 2;
    let total_samples = beat_duration * sequence_length as usize;
    let mut mixed_buffer = vec![(0i16, 0i16); total_samples];

    // Load and decode all audio samples
    let mut decoded_samples = BTreeMap::new();
    for (index, file_name) in selected_samples.iter() {
        let path = Path::new("drumKits/TR-808 Kit").join(file_name);
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        let samples: Vec<i16> = source.convert_samples().collect();
        decoded_samples.insert(*index, samples);
    }
    // Mix samples according to the beat pattern
    for (file_index, file_pattern) in beat_pattern.iter().enumerate() {
        if let Some(samples) = decoded_samples.get(&file_index) {
            for (beat, &active) in file_pattern.iter().enumerate() {
                if active {
                    let start = beat * beat_duration;
                    for (i, &sample) in samples.iter().enumerate().step_by(2) {
                        if start + i < total_samples {
                            mixed_buffer[start + i].0 =
                                mixed_buffer[start + i].0.saturating_add(sample);
                            if i + 1 < samples.len() {
                                mixed_buffer[start + i].1 =
                                    mixed_buffer[start + i].1.saturating_add(samples[i + 1]);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    // Write mixed buffer to WAV file
    for &(left, right) in &mixed_buffer {
        writer.write_sample(left)?;
        writer.write_sample(right)?;
    }

    writer.finalize()?;
    Ok(())
}
