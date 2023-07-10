use std::collections::{BTreeMap, HashMap};
use tracing::{
    field::{Field, ValueSet},
    span, Event, Subscriber,
};
use tracing_subscriber::{
    field::Visit, layer::Context, prelude::__tracing_subscriber_SubscriberExt, registry,
    registry::LookupSpan, util::SubscriberInitExt, EnvFilter, Layer,
};

struct CustomLayer;

struct JsonVisitor<'a> {
    map: &'a mut BTreeMap<String, serde_json::Value>,
}

pub fn init_logger() {
    registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors are at TRACE level
            "chat_rs=debug,axum::rejection=trace".into()
        }))
        // .with(tracing_subscriber::fmt::layer().json())
        .with(CustomLayer)
        .init();
}

impl<S> Layer<S> for CustomLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, _id: &span::Id, _ctx: Context<'_, S>) {
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor { map: &mut fields };
        attrs.values().record(&mut visitor);
        let ctx = _ctx.span(_id).unwrap();
        // println!("{:?}", );
        // ctx.span(id).unwrap().extensions_mut().insert()
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Convert field values into JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor { map: &mut fields };
        event.record(&mut visitor);

        let span = ctx.event_span(event).unwrap();
        let mut output: HashMap<String, serde_json::Value> = HashMap::new();
        // Log function name
        output.insert(
            "function".to_string(),
            serde_json::Value::String(span.name().to_string()),
        );
        // Log the custom message we typed
        fields.iter().for_each(|(key, value)| {
            output.insert(key.clone(), value.clone());
        });
        // Log the log message level
        output.insert(
            "level".to_string(),
            serde_json::Value::String(event.metadata().level().to_string().to_lowercase()),
        );
        // Log the location / module where the span / event happens
        output.insert(
            "target".to_string(),
            serde_json::Value::String(event.metadata().target().to_string()),
        );
        // Log the function parameters if there are some
        let mut parameters: HashMap<String, serde_json::Value> = HashMap::new();
        span.fields().iter().for_each(|field| {
            parameters.insert(
                field.name().to_string(),
                serde_json::Value::String("".to_string()),
            );
        });
        output.insert("fields".to_string(), serde_json::json!(parameters));

        println!("{:?}", span.fields());

        // span.extensions()
        //     .get::<>()
        //     .map(|value| serde_json::json!(value));
        // let i = binding;
        // match i {
        //     Some(haha) => println!("{:#?}", haha),
        //     None => (),
        // }
        // println!("{:?}", span.extensions().get::<CustomSpanParameterValues>());

        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!(output)).unwrap()
        );
    }
}

impl<'a> Visit for JsonVisitor<'a> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.map
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.map
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.map
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.map
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.map
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.map.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.map.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }
}
