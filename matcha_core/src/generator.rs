use crate::error::Result;
use ort::{GraphOptimizationLevel, Session};

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
}
