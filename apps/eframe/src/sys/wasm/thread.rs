use super::time::Duration;

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    tokio_with_wasm::spawn(future);
}

pub async fn sleep(duration: Duration) {
    tokio_with_wasm::time::sleep(duration).await;
}
