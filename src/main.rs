#[macro_use]
extern crate tracing;
// extern crate tracing_subscriber;

use itertools::Itertools;
use opentelemetry::sdk::export::trace::stdout;
use opentelemetry_otlp::WithExportConfig;
use replex::cache::*;
use replex::config::Config;
use replex::logging::*;
use replex::models::*;
use replex::routes::*;
use replex::plex_client::*;
// use replex::proxy::PlexProxy;
use replex::transform::*;
use replex::url::*;
use replex::utils::*;
use salvo::cache::{Cache, MemoryStore};
use salvo::compression::Compression;
use salvo::cors::Cors;
use salvo::prelude::*;
use salvo::proxy::Proxy as SalvoProxy;
use std::time::Duration;
use tonic::metadata::MetadataMap;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    let config: Config = Config::figment().extract().unwrap();
    if config.host.is_none() {
        tracing::error!("REPLEX_HOST is required. Exiting");
        return;
    }

    let fmt_layer = tracing_subscriber::fmt::layer();
    let console_layer = match config.enable_console {
        true => Some(console_subscriber::spawn()),
        false => None,
    };

    let otlp_layer = if config.newrelic_api_key.is_some() {
        let mut map = MetadataMap::with_capacity(3);
        map.insert(
            "api-key",
            config.newrelic_api_key.unwrap().parse().unwrap(),
        );
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_tls_config(Default::default())
                    .with_endpoint(
                        "https://otlp.eu01.nr-data.net:443/v1/traces",
                    )
                    .with_metadata(map)
                    .with_timeout(Duration::from_secs(3)),
            )
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap();
        Some(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(console_layer)
        .with(otlp_layer)
        .with(fmt_layer)
        .init();


    let router = route();
    if config.ssl_enable && config.ssl_domain.is_some() {
        let acceptor = TcpListener::new("0.0.0.0:443")
            .acme()
            .cache_path("/data/acme/letsencrypt")
            .add_domain(config.ssl_domain.unwrap())
            .bind()
            .await;
        Server::new(acceptor).serve(router).await;
    } else {
        let acceptor = TcpListener::new("0.0.0.0:80").bind().await;
        Server::new(acceptor).serve(router).await;
    }
}
