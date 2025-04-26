use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    
    #[cfg(unix)]
    let mut terminate = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");

    #[cfg(not(unix))]
    let mut terminate = None;  // fallback dummy

    tokio::select! {
        _ = ctrl_c => {},
        _ = async {
            #[cfg(unix)]
            {
                terminate.recv().await;
            }
            #[cfg(not(unix))]
            {
                std::future::pending::<()>().await;
            }
        } => {},
    }

    println!("Shutdown signal received, exiting...");
}
