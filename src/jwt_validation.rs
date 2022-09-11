use std::convert::TryInto;
use std::ops::ControlFlow;

use apollo_router::graphql;
use apollo_router::services::supergraph;
use apollo_router::{
    layers::ServiceBuilderExt,
    plugin::{Plugin, PluginInit},
    register_plugin,
};
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::{BoxError, ServiceBuilder, ServiceExt};
use tracing::debug;

#[derive(Deserialize, JsonSchema)]
struct JwtValidationConfig {
    secret_key: String,
}

#[derive(Debug)]
struct JwtValidation {
    secret_key: String,
}

#[async_trait::async_trait]
impl Plugin for JwtValidation {
    type Config = JwtValidationConfig;

    async fn new(init: PluginInit<Self::Config>) -> Result<Self, BoxError> {
        Ok(JwtValidation {
            secret_key: init.config.secret_key,
        })
    }
}

register_plugin!("demo", "jwt_validation", JwtValidation);

// #[cfg(test)]
// mod tests {
//
//     #[tokio::test]
//     async fn plugin_registered() {
//         let config = serde_json::json!({
//             "plugins": {
//                 "demo.jwt_validation": {
//                     "secret_key": "example"
//                 }
//             }
//         });
//
//         apollo_router::TestHarness::builder()
//             .configuration_json(config)
//             .unwrap()
//             .build()
//             .await
//             .unwrap();
//     }
// }
