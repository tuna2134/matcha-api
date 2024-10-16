use matcha_core::{MatchaGenerator, txt2seq, TextPreprocessor, Scale, Vocoder};
use pyo3::prelude::*;
use numpy::{PyReadonlyArray3, PyArray3};

#[pyclass]
pub struct Matcha {
    generator: MatchaGenerator,
    vocoder: Vocoder,
    pp: TextPreprocessor,
}

#[pymethods]
impl Matcha {
    #[new]
    fn new(matcha: Vec<u8>, vocoder: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Matcha {
            generator: MatchaGenerator::new(matcha)?,
            pp: TextPreprocessor::new()?,
            vocoder: Vocoder::new(vocoder)?,
        })
    }

    fn preprocess(&self, text: &str) -> anyhow::Result<Vec<i64>> {
        let clean_text = self.pp.g2p(text)?;
        Ok(txt2seq(clean_text)?)
    }

    fn synthesise<'a>(&'a self, py: Python<'a>, symbols: Vec<i64>) -> anyhow::Result<(Bound<PyArray3<f32>>, i64)> {
        let (mel, mel_lengths) = self.generator.synthesise(symbols, Scale::default())?;
        Ok((PyArray3::from_owned_array_bound(py, mel), mel_lengths))
    }

    fn decode(&self, mel: PyReadonlyArray3<f32>, mel_lengths: i64) -> anyhow::Result<Vec<u8>> {
        let mel = mel.as_array();
        let data = self.vocoder.decode(mel.to_owned(), mel_lengths)?;
        Ok(data)
    }
}