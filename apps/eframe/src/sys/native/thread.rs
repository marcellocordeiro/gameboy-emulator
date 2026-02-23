use super::time::Duration;

pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(future);
}

pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}
