use chrono::Utc;
use serde_json::{json, Map, Value};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::field::{Field, Visit};
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub struct SeqLayer {
    sender: mpsc::UnboundedSender<String>,
}

impl SeqLayer {
    pub fn init(url: String, api_key: Option<String>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(flush_task(url, api_key, rx));
        Self { sender: tx }
    }
}

#[derive(Default)]
struct JsonVisitor {
    fields: Map<String, Value>,
}

impl Visit for JsonVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name().into(), Value::String(value.into()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields
            .insert(field.name().into(), Value::String(format!("{:?}", value)));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields.insert(field.name().into(), json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields.insert(field.name().into(), json!(value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields.insert(field.name().into(), json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields.insert(field.name().into(), Value::Bool(value));
    }
}

fn level_to_seq(level: &tracing::Level) -> &'static str {
    match *level {
        tracing::Level::TRACE => "Verbose",
        tracing::Level::DEBUG => "Debug",
        tracing::Level::INFO => "Information",
        tracing::Level::WARN => "Warning",
        tracing::Level::ERROR => "Error",
    }
}

impl<S> Layer<S> for SeqLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let meta = event.metadata();
        let mut visitor = JsonVisitor::default();
        event.record(&mut visitor);

        let message = visitor
            .fields
            .remove("message")
            .and_then(|v| match v {
                Value::String(s) => Some(s),
                _ => None,
            })
            .unwrap_or_default();

        let mut obj = Map::new();
        obj.insert(
            "@t".into(),
            Value::String(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        );
        obj.insert("@mt".into(), Value::String(message));
        obj.insert(
            "@l".into(),
            Value::String(level_to_seq(meta.level()).into()),
        );

        if let Some(module) = meta.module_path() {
            obj.insert("SourceContext".into(), Value::String(module.into()));
        }

        // Walk parent spans and collect their fields
        if let Some(scope) = ctx.event_scope(event) {
            let mut span_names = Vec::new();
            for span in scope.from_root() {
                span_names.push(span.name().to_string());
                let extensions = span.extensions();
                if let Some(fields) = extensions.get::<Map<String, Value>>() {
                    for (k, v) in fields {
                        obj.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
            }
            if !span_names.is_empty() {
                obj.insert(
                    "SpanPath".into(),
                    Value::String(span_names.join(" > ")),
                );
            }
        }

        for (k, v) in visitor.fields {
            obj.insert(k, v);
        }

        if let Ok(json) = serde_json::to_string(&Value::Object(obj)) {
            let _ = self.sender.send(json);
        }
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: Context<'_, S>,
    ) {
        let mut visitor = JsonVisitor::default();
        attrs.record(&mut visitor);
        if let Some(span) = ctx.span(id) {
            span.extensions_mut().insert(visitor.fields);
        }
    }
}

async fn flush_task(
    url: String,
    api_key: Option<String>,
    mut rx: mpsc::UnboundedReceiver<String>,
) {
    let client = reqwest::Client::new();
    let endpoint = format!("{}/api/events/raw", url.trim_end_matches('/'));
    let mut buffer: Vec<String> = Vec::new();

    loop {
        match tokio::time::timeout(Duration::from_millis(500), rx.recv()).await {
            Ok(Some(event)) => {
                buffer.push(event);
                while let Ok(event) = rx.try_recv() {
                    buffer.push(event);
                }
            }
            Ok(None) => {
                if !buffer.is_empty() {
                    let _ = send_batch(&client, &endpoint, &api_key, &buffer).await;
                }
                break;
            }
            Err(_) => {}
        }

        if !buffer.is_empty() {
            let _ = send_batch(&client, &endpoint, &api_key, &buffer).await;
            buffer.clear();
        }
    }
}

async fn send_batch(
    client: &reqwest::Client,
    endpoint: &str,
    api_key: &Option<String>,
    events: &[String],
) -> Result<(), reqwest::Error> {
    let body = events.join("\n");
    let mut req = client
        .post(endpoint)
        .header("Content-Type", "application/vnd.serilog.clef")
        .body(body);
    if let Some(ref key) = api_key {
        if !key.is_empty() {
            req = req.header("X-Seq-ApiKey", key);
        }
    }
    req.send().await?;
    Ok(())
}
