use super::time::Duration;

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future);
}

pub async fn sleep(duration: Duration) {
    wasmtimer::tokio::sleep(duration).await;
}
