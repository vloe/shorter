use tokio::signal;

pub async fn axum_shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("signal received, starting graceful shutdown");
}
