use ort::Session;
use ndarray::{Array3, arr1};

use crate::error::Result;
use crate::general::load_model;

pub struct Vocoder {
    session: Session,
}

impl Vocoder {
    pub fn new<P: AsRef<[u8]>>(model: P) -> Result<Self> {
        let session = load_model(model)?;
        Ok(Self { session })
    }

    pub fn decode(&self, mel: Array3<f32>, mel_lengths: i64) -> Result<()> {
        let outputs = self.session.run(ort::inputs![
            "mel" => mel,
            "mel_lengths" => arr1(&[mel_lengths]),
        ]?)?;
        Ok(())
    }
}