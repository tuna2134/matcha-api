use crate::error::Result;
use crate::general::load_model;
use crate::utils::intersperse;

use ndarray::{arr1, Array2, Array3, Ix3};
use ort::Session;

pub struct MatchaGenerator {
    session: Session,
}

impl MatchaGenerator {
    pub fn new<P: AsRef<[u8]>>(model: P) -> Result<Self> {
        let session = load_model(model)?;
        Ok(Self { session })
    }

    pub fn synthesise(&self, symbols: Vec<i64>, scale: Scale) -> Result<(Array3<f32>, i64)> {
        let symbols = intersperse(symbols, 0);
        let x = Array2::from_shape_vec((1, symbols.len()), symbols.clone())?;
        let x_lengths = arr1(&[symbols.len() as i64]);
        let outputs = self.session.run(ort::inputs![
            "x" => x,
            "x_lengths" => x_lengths,
            "scales" => scale.to_ndarray(),
        ]?)?;
        let audio_array = outputs["mel"].try_extract_tensor::<f32>()?.into_dimensionality::<Ix3>()?.to_owned();
        let mel_lengths = outputs["mel_lengths"].try_extract_tensor::<i64>()?;

        Ok((
            audio_array,
            mel_lengths[0],
        ))
    }
}

pub struct Scale {
    pub temperature: f32,
    pub speaking_rate: f32,
}

impl Scale {
    pub fn to_ndarray(&self) -> ndarray::Array1<f32> {
        arr1(&[self.temperature, self.speaking_rate])
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            temperature: 0.677,
            speaking_rate: 1.0,
        }
    }
}
