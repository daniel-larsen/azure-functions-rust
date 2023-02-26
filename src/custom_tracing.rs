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
        let mut visitor = CustomVisitor::new();
        event.record(&mut visitor);

        if *event.metadata().level() <= self.level {
            self.events.lock().unwrap().push(format!(
                "{} {:?} {}",
                event.metadata().level(),
                visitor.0,
                event.metadata().name()
            ));
        }
    }
}

struct CustomVisitor(Vec<String>);

impl CustomVisitor {
    fn new() -> Self {
        Self { 0: Vec::new() }
    }
}

impl<'a> tracing::field::Visit for CustomVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.push(format!("{}: {} ", field.name(), value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.push(format!("{}: {:?} ", field.name(), value));
    }
}
