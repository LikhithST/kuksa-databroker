#[cfg(feature="otel")]
use {
 opentelemetry::{KeyValue, runtime},
 opentelemetry::sdk::{Resource, trace},
 opentelemetry::trace::TraceError,
 opentelemetry_otlp::WithExportConfig,
 std::env
};

#[cfg(feature="otel")]
pub fn init_trace() -> Result<trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(env::var("OTEL_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string())),
        ).with_batch_config(trace::BatchConfig::default()) // to change default of max_queue_size use .with_max_queue_size(8192) or set env OTEL_BSP_MAX_QUEUE_SIZE, by default it is set to 2_048
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "kuksa-rust-app",
            )])),
        )
        .install_batch(runtime::Tokio)
}
