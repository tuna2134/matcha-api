use crate::error::Result;
use crate::utils::intersperse;

use ort::{GraphOptimizationLevel, Session};
use ndarray::{arr1, Array2};

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
        let symbols = intersperse(symbols, 0);
        let x = Array2::from_shape_vec((1, symbols.len()), symbols.clone())?;
        let x_lengths = arr1(&[symbols.len() as i64]);
        let outputs = self.session.run(ort::inputs![
            "x" => x,
            "x_lengths" => x_lengths,
            "scales" => scale.to_ndarray(),
        ]?)?;
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