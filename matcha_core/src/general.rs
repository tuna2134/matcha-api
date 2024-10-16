use crate::error::Result;
use ort::{GraphOptimizationLevel, Session};

pub fn load_model<T: AsRef<[u8]>>(model: T) -> Result<Session> {
    let mut exp = Vec::new();
    #[cfg(feature = "cuda")]
    {
        let cuda = ort::CUDAExecutionProvider::default()
            .with_conv_algorithm_search(ort::CUDAExecutionProviderCuDNNConvAlgoSearch::Default);
        exp.push(cuda.build());
    }
    exp.push(ort::CPUExecutionProvider::default().build());
    let session = Session::builder()?
        .with_execution_providers(exp)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_parallel_execution(true)?
        .with_inter_threads(num_cpus::get_physical())?
        .commit_from_memory(model.as_ref())?;
    Ok(session)
}
