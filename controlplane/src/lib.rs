/*
Copyright 2024 The Kubernetes Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use kube::Client;
use thiserror::Error;

pub mod gateway_controller;
pub mod gateway_utils;
pub mod gatewayclass_controller;
pub mod gatewayclass_utils;
mod traits;
pub mod utils;

// Context for our reconciler
#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("kube error: {0}")]
    KubeError(#[source] kube::Error),
    #[error("invalid configuration: `{0}`")]
    InvalidConfigError(String),
    #[error("error reconciling loadbalancer service: `{0}`")]
    LoadBalancerError(String),
    #[error("error querying Gateway API CRDs: `{0}`; are the CRDs installed?")]
    CRDNotFoundError(#[source] kube::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub const GATEWAY_CLASS_CONTROLLER_NAME: &str = "gateway.networking.k8s.io/blixt";
pub const BLIXT_FIELD_MANAGER: &str = "blixt-field-manager";
pub const GATEWAY_SERVICE_LABEL: &str = "blixt.gateway.networking.k8s.io/owned-by-gateway";

pub struct NamespacedName {
    pub name: String,
    pub namespace: String,
}
