use ndarray::Array2;

use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Cursor;

use crate::error::Result;

pub fn intersperse(slice: Vec<i64>, item: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(slice.len() * 2 + 1);
    for val in slice.iter() {
        result.push(item);
        result.push(*val);
    }
    result.push(item);
    result
}

pub fn array_to_vec(audio_array: Array2<f32>) -> Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 20050,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut cursor, spec)?;
    for i in audio_array.outer_iter() {
        for &sample in i.iter() {
            writer.write_sample(sample)?;
        }
    }
    writer.finalize()?;
    Ok(cursor.into_inner())
}
