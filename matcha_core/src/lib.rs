pub mod error;
pub mod generator;
pub mod text;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> error::Result<()> {
        let pp = text::TextPreprocessor::new()?;
        let result = text::txt2seq("こんにちは", pp)?;
        println!("{:?}", result);
        Ok(())
    }
}
