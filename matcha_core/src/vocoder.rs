use ndarray::{arr1, Array2, Array3, Ix2};
use ort::Session;

use crate::error::Result;
use crate::general::load_model;
use crate::utils::array_to_vec;

pub struct Vocoder {
    session: Session,
}

impl Vocoder {
    pub fn new<P: AsRef<[u8]>>(model: P) -> Result<Self> {
        let session = load_model(model)?;
        Ok(Self { session })
    }

    pub fn decode(&self, mel: Array3<f32>, mel_lengths: i64) -> Result<Vec<u8>> {
        let outputs = self.session.run(ort::inputs![
            "mel" => mel,
            "mel_lengths" => arr1(&[mel_lengths]),
        ]?)?;
        let wav: Array2<f32> = outputs["wav"]
            .try_extract_tensor::<f32>()?
            .into_dimensionality::<Ix2>()
            .unwrap()
            .to_owned();
        array_to_vec(wav)
    }
}
