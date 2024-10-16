pub mod error;
pub mod general;
pub mod generator;
pub mod text;
pub mod utils;

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
        let result = text::txt2seq("こんにちは", pp)?;
        println!("{:?}", result);
        let generator = generator::MatchaGenerator::new(fs::read("model.onnx")?)?;
        generator.synthesise(result, generator::Scale::default())?;
        Ok(())
    }
}
