use configs::Configurations;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{trace::TraceError, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::prelude::*;

mod app;
mod configs;
mod database;
pub mod models;
pub mod schema;
mod shutdown;

fn init_tracer(config: &Configurations) -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(config.tracing.host.clone()),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                config.service.name.clone(),
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tokio::main]
async fn main() {
    // Load the configurations
    let config = Configurations::new().expect("Error loading the configurations.");

    // initialize tracing
    let tracer = init_tracer(&config).expect("Failed to initialize tracer.");
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.logger.level))
        .with(fmt_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let app_state = database::get_connection_pool(&config);
    let app = app::create_app(app_state);

    let address: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect("Unable to parse socket address");
    let rx = shutdown::register();

    info!("Starting server on {:?}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok(); // This will block until a shutdown signal is received
            info!("Handling graceful shutdown");
            info!("Close resources, drain and shutdown event handler... etc");
            shutdown_tracer_provider();
        })
        .await
        .expect("Failed to start server");
}
