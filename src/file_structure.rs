use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ClusterConfig {
  #[serde(rename = "certificate-authority-data")]
  pub(crate) certificate_authority_data: String,
  pub(crate) server: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ClustersConfig {
  pub(crate) cluster: ClusterConfig,
  pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ContextConfig {
  pub(crate) cluster: String,
  pub(crate) user: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct ContextsConfig {
  pub(crate) context: ContextConfig,
  pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct UserConfig {
  #[serde(rename = "client-certificate-data")]
  pub(crate) client_certificate_data: String,
  #[serde(rename = "client-key-data")]
  pub(crate) client_key_data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct UsersConfig {
  pub(crate) user: UserConfig,
  pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PreferencesConfig {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct KubeConfig {
  #[serde(rename = "apiVersion")]
  pub(crate) api_version: String,
  pub(crate) clusters: Vec<ClustersConfig>,
  pub(crate) contexts: Vec<ContextsConfig>,
  pub(crate) preferences: PreferencesConfig,
  #[serde(rename = "current-context")]
  pub(crate) current_context: String,
  pub(crate) kind: String,
  pub(crate) users: Vec<UsersConfig>,
}
