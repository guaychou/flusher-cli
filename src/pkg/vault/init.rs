use {crate::pkg::vault::model::VaultResponse, std::env};

pub fn hit_vault(
    req_client: &reqwest::blocking::Client,
    secret_path: Option<&str>,
) -> Result<VaultResponse, reqwest::Error> {
    let vault_token = env::var("VAULT_TOKEN").expect("Cannot get VAULT_TOKEN env variable");
    let vault_addr = match env::var("VAULT_ADDR") {
        Ok(addr) => addr,
        Err(_) => "http://127.0.0.1:8200".to_string(),
    };
    let vault_config = env::var("VAULT_CONFIG").expect("Cannot get VAULT_CONFIG env variable");

    // if Secret path has value then we will use it , in example to passing the app vault name, if none then we will use the config/flusher
    let secret_path = match secret_path {
        Some(data) => data,
        None => vault_config.as_str(),
    };

    let vault_complete_url = format!("{}{}{}", vault_addr, "/v1/secret/", secret_path);
    let resp = req_client
        .get(&vault_complete_url)
        .header("X-Vault-Token", &vault_token)
        .send()
        .expect("Vault initialization error, cannot reach the vault server");

    match resp.error_for_status() {
        Ok(resp) => resp.json(),
        Err(e) => return Err(e),
    }
}
