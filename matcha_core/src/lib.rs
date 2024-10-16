pub mod error;
pub mod general;
pub mod generator;
pub mod text;
pub mod utils;
pub mod vocoder;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let pp = text::TextPreprocessor::new()?;
        let result = text::txt2seq("こんにちは、今日はいい天気ですね。", pp)?;
        let generator = generator::MatchaGenerator::new(fs::read("model.onnx")?)?;
        let (mel, mel_lengths) = generator.synthesise(result, generator::Scale::default())?;
        let vocoder = vocoder::Vocoder::new(fs::read("vocoder.onnx")?)?;
        let data = vocoder.decode(mel, mel_lengths)?;
        fs::write("output.wav", data)?;
        Ok(())
    }
}
