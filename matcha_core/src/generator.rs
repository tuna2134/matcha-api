use crate::error::Result;
use crate::utils::intersperse;

use ort::{GraphOptimizationLevel, Session};
use ndarray::arr1;

pub struct MatchaGenerator {
    session: Session,
}

impl MatchaGenerator {
    pub fn new<P: AsRef<[u8]>>(model: P) -> Result<Self> {
        let mut exp = Vec::new();
        exp.push(ort::CPUExecutionProvider::default().build());
        let session = Session::builder()?
            .with_execution_providers(exp)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_parallel_execution(true)?
            .with_inter_threads(num_cpus::get_physical())?
            .commit_from_memory(model.as_ref())?;
        Ok(Self { session })
    }

    pub fn synthesise(&self, symbols: Vec<i64>, scale: Scale) -> Result<()> {
        let x = arr1(&intersperse(symbols, 0)).to_owned();
        let x_lengths = arr1(&[x.len() as i64]).to_owned();
        let outputs = self.session.run(ort::inputs![
            "x" => x,
            "x_lengths" => x_lengths,
            "scale" => scale.to_ndarray(),
        ]?);
        Ok(())
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