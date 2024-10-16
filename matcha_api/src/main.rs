use axum::{
    extract::State,
    http::header::CONTENT_TYPE,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use matcha_core::{txt2seq, MatchaGenerator, Scale, TextPreprocessor, Vocoder};
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio::fs;

mod error;
use error::AppResult;

fn temperature_default() -> f32 {
    0.677
}

fn speaking_rate_default() -> f32 {
    1.0
}

#[derive(Deserialize)]
struct SynthesizeRequest {
    text: String,
    #[serde(default = "temperature_default")]
    temperature: f32,
    #[serde(default = "speaking_rate_default")]
    speaking_rate: f32,
}

#[derive(Clone)]
struct AppState {
    pub pp: Arc<TextPreprocessor>,
    pub generator: Arc<MatchaGenerator>,
    pub vocoder: Arc<Vocoder>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let pp = Arc::new(TextPreprocessor::new()?);
        let generator = Arc::new(MatchaGenerator::new(fs::read("model.onnx").await?)?);
        let vocoder = Arc::new(Vocoder::new(fs::read("vocoder.onnx").await?)?);
        Ok(Self {
            pp,
            generator,
            vocoder,
        })
    }
}

async fn synthesize(
    State(state): State<AppState>,
    Json(SynthesizeRequest {
        text,
        temperature,
        speaking_rate,
    }): Json<SynthesizeRequest>,
) -> AppResult<impl IntoResponse> {
    let buffer = {
        let clean_text = state.pp.g2p(&text)?;
        let symbols = txt2seq(clean_text)?;
        let (mel, mel_lengths) = state.generator.synthesise(
            symbols,
            Scale {
                temperature,
                speaking_rate,
            },
        )?;
        let data = state.vocoder.decode(mel, mel_lengths)?;
        data
    };
    Ok(([(CONTENT_TYPE, "audio/wav")], buffer))
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv_override().ok();
    // env_logger::init();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/synthesize", post(synthesize))
        .with_state(AppState::new().await?);
    let addr = env::var("ADDR").unwrap_or("0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    log::info!("Listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
