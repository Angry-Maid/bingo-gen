use std::{path::PathBuf, sync::Arc};

use bridge::handle::{BackendHandle, BackendReceiver, FrontendHandle};

pub fn start(
    export_dir: PathBuf,
    send: FrontendHandle,
    self_handle: BackendHandle,
    recv: BackendReceiver,
) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime");

    let mut state = BackendState {
        self_handle,
        send,
        export_dir: Arc::new(export_dir),
    };

    runtime.spawn(state.start(recv));

    std::mem::forget(runtime);
}

#[derive(Clone)]
pub struct BackendState {
    pub self_handle: BackendHandle,
    pub send: FrontendHandle,
    pub export_dir: Arc<PathBuf>,
}

impl BackendState {
    async fn start(self, recv: BackendReceiver) {
        log::info!("Starting backend");

        self.handle(recv).await;
    }

    async fn handle(mut self, mut backend_recv: BackendReceiver) {
        loop {
            tokio::select! {
                message = backend_recv.recv() => {
                    if let Some(msg) = message {
                        self.handle_message(msg).await;
                    } else {
                        log::info!("Backend receiver shut down");
                        break;
                    }
                }
            }
        }
    }
}
