use anyhow::Ok;
use apiexts::CustomResourceDefinition;
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::{
    api::core::v1::Node, apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts, serde,
};
use kube::{
    api::{Api, ListParams, Patch, PatchParams, ResourceExt},
    runtime::wait::{await_condition, conditions},
    runtime::{watcher, WatchStreamExt},
    Client, CustomResource, CustomResourceExt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "kata.dev", version = "v1", kind = "Model", namespaced)]
#[kube(status = "ModelStatus")]
struct ModelSpec {
    pub name: String,
    pub description: String,
    pub query: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct ModelStatus {
    pub is_ok: bool,
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "kata.dev", version = "v1", kind = "Backend", namespaced)]
#[kube(status = "BackendStatus")]
struct BackendSpec {
    pub name: String,
    pub class: String,
    pub connection_params: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct BackendStatus {
    pub is_ok: bool,
}

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "kata.dev", version = "v1", kind = "Task", namespaced)]
#[kube(status = "TaskStatus")]
struct TaskSpec {
    // pub name: String,
    pub model_name: String,
    pub backend_name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct TaskStatus {
    pub is_ok: bool,
}

const CRD_NAME_MODELS: &str = "models.kata.dev";
const CRD_NAME_BACKENDS: &str = "backends.kata.dev";
const CRD_NAME_TASKS: &str = "tasks.kata.dev";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    // Create CRDs
    apply_crd(&client, CRD_NAME_MODELS, Model::crd()).await?;
    apply_crd(&client, CRD_NAME_BACKENDS, Backend::crd()).await?;
    apply_crd(&client, CRD_NAME_TASKS, Task::crd()).await?;

    // Get the current topology

    // Watch modifications to the topology

    Ok(())
}

async fn apply_crd(
    client: &Client,
    crd_name: &str,
    crd: CustomResourceDefinition,
) -> anyhow::Result<()> {
    let crd_apply_params = PatchParams::apply("crd_apply").force();
    let crd_client: Api<CustomResourceDefinition> = Api::all(client.clone());

    crd_client
        .patch(crd_name, &crd_apply_params, &Patch::Apply(crd))
        .await?;

    info!("Creating CRD: {}", crd_name);

    let stablish = await_condition(crd_client, crd_name, conditions::is_crd_established());

    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), stablish).await?;

    Ok(())
}
