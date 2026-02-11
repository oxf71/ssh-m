use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpVault {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpItem {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(default)]
    pub vault: OpItemVault,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpItemVault {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpItemDetail {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(default)]
    pub fields: Vec<OpField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpField {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub value: String,
    #[serde(default, rename = "type")]
    pub field_type: String,
    #[serde(default)]
    pub section: Option<OpFieldSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpFieldSection {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpStatus {
    pub cli_installed: bool,
    pub cli_version: Option<String>,
    pub signed_in: bool,
    pub accounts: Vec<String>,
}
