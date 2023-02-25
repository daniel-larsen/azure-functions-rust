use std::sync::{Arc, Mutex};
use tracing::Level;
use tracing_subscriber::Layer;

#[derive(Clone)]
pub struct CustomLayer {
    events: Arc<Mutex<Vec<String>>>,
    level: Level,
}

impl CustomLayer {
    pub fn new(level: Level) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            level,
        }
    }

    pub fn get(&self) -> Vec<String> {
        self.events.lock().unwrap().to_vec()
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
        if *event.metadata().level() > self.level {
            self.events.lock().unwrap().push(format!("{:#?}", event));
        }
    }
}
