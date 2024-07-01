use hound::{WavSpec, WavWriter};
use rodio::{Decoder, Source};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::ui::drum_machine_page::SampleFolder;

pub fn record_pattern(
    beat_pattern: &Vec<Vec<bool>>,
    audio_files: &Vec<String>,
    bpm: u32,
    sequence_length: u32,
    selected_samples: &BTreeMap<usize, HashMap<String, SampleFolder>>,
    output_file: &str,
    root_sample_path: &str,
    beat_scale: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path = Path::new("recorded_patterns").join(output_file);
    let mut writer = WavWriter::create(path, spec)?;

    let beat_duration =
        (60.0 / bpm as f32 * spec.sample_rate as f32) as usize / beat_scale as usize;
    let total_samples = beat_duration * sequence_length as usize;
    let mut mixed_buffer = vec![(0i16, 0i16); total_samples];

    // Load and decode all audio samples
    let mut decoded_samples = BTreeMap::new();
    for (index, file_map) in selected_samples.iter() {
        let sample_folder = file_map.values().next().unwrap().to_string();
        let full_path = root_sample_path.to_string() + &sample_folder;

        let path = Path::new(&full_path).join(file_map.keys().next().unwrap());
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
