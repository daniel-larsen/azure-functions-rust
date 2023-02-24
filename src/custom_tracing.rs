use std::sync::{Arc, Mutex};
use tracing_subscriber::Layer;

#[derive(Clone)]
pub struct CustomLayer(Arc<Mutex<Vec<String>>>);

impl CustomLayer {
    pub fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get(&self) -> Vec<String> {
        self.0.lock().unwrap().to_vec()
    }
}

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.lock().unwrap().push(format!("{:#?}", event));
    }
}
