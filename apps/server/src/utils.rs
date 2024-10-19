use crate::error::AppError;
use std::env;
use tokio::signal;

pub async fn axum_shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("signal received, starting graceful shutdown");
}

pub fn get_env(key: &'static str) -> Result<String, AppError> {
    env::var(key).map_err(|_| AppError::EnvNotSet(key))
}
