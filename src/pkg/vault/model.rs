use {
    crate::pkg::flusher::model::{FlusherVault, FlusherVaultApps},
    enum_as_inner::EnumAsInner,
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct VaultResponse {
    pub data: VaultResponseType,
}

#[derive(Debug, Deserialize, EnumAsInner)]
#[serde(untagged)]
pub enum VaultResponseType {
    FlusherVaultApps(FlusherVaultApps),
    FlusherVault(FlusherVault),
}
