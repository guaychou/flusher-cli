use {
    crate::pkg::flusher::model::*,
    crate::pkg::vault::init::hit_vault,
    log::{debug, error, info},
    serde_json::json,
};

/*
   function to initiate the hit flusher . there are 3 steps
   1. Health check the flusher first
   2. Get Vault Credential for redis password and basic auth for flusher
   3. Hit the Flusher with any of opeartion defined in config file (FlushAll/FlushAllAsync/DelKeyByPrefix)
*/

pub fn hit_flusher(flusher_address: &str, dry_run: &bool, config_path: &str, app_name: &str) {
    // Create HTTP client with reqwest
    let req_client = reqwest::blocking::Client::new();

    // Run Health check for flusher , if flusher not healthy this run will exit;
    match health_check_first(flusher_address, &req_client) {
        Ok(res) => info!("{}", res),
        Err(e) => {
            error!("{}", e.to_string());
            std::process::exit(1)
        }
    };
    info!("Hit flusher vault to retrive flusher authentication credentials");
    // Initialize vault get fluser auth
    let vault_response = match hit_vault(&req_client, None) {
        Ok(res) => res,
        Err(e) => {
            error!("{}", e.to_string());
            std::process::exit(1)
        }
    };

    let flusher_auth = vault_response.data.as_flusher_vault().unwrap();
    // Reading the yaml config. . .
    let mut data = read_yaml(config_path);
    // Check if we run into dry run mode or not , usually dry run mode will enable when PR is built
    if *dry_run {
        info!("Flusher cli run in dry run mode");
        debug!("Debug data {:#?}", data);
        info!("Flusher cli run succesfully");
        std::process::exit(0);
    }

    // Checking if developer enable the password of redis then we will get it from vault
    if *data.get_auth() {
        info!("Get the redis password from application vault");
        let vault_response = match hit_vault(&req_client, Some(app_name)) {
            Ok(res) => res,
            Err(e) => {
                error!("ERROR : {:#?}", e);
                std::process::exit(1)
            }
        };
        let redis_password = match vault_response.data.as_flusher_vault_apps() {
            Some(data) => data.get_flusher_password(),
            None => flusher_auth.get_flusher_default_password(),
        };
        // Setting the redis password if redis auth enabled from vault
        data.set_password(Some(redis_password.to_string()));
        //Check if developer add master redis name to config file if yess add the redis password into sentinel password too.
        match data.get_master() {
            Some(_) => data.set_sentinel_password(Some(redis_password.to_string())),
            None => data.set_sentinel_password(None),
        };
    }

    // Hit the flusher with certain operation !
    info!("Running the operation: {:#?}", data.get_operation());
    match run_operation(flusher_address, &data, &req_client, flusher_auth) {
        FlusherResult::Ok(data) => {
            info!("{}", data);
            info!("Flusher cli run successfuly");
            std::process::exit(0)
        }
        FlusherResult::Err(e) => {
            error!("{}", e);
            std::process::exit(1)
        }
    };
}

fn run_operation(
    flusher_address: &str,
    flusher_data: &RedisData,
    req_client: &reqwest::blocking::Client,
    flusher_auth: &FlusherVault,
) -> FlusherResult {
    let data = json!({ "data": flusher_data });
    let resp = req_client
        .post(flusher_address.to_owned() + flusher_data.get_operation().suffix())
        .json(&data)
        .basic_auth(
            &flusher_auth.get_username(),
            Some(&flusher_auth.get_password()),
        )
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .expect("Request to Flusher cannot be send, seems like network issue.");

    resp.json::<FlusherResult>().unwrap()
}

// Function to healthcheck flusher
fn health_check_first(
    flusher_address: &str,
    req_client: &reqwest::blocking::Client,
) -> Result<FlusherHealthyResponse, reqwest::Error> {
    let resp = req_client
        .get(flusher_address.to_owned() + "/api/v1/health")
        .send()
        .expect("Cannot do health check, request not send");

    match resp.error_for_status_ref() {
        Ok(_) => return Ok(resp.json::<FlusherHealthyResponse>().unwrap()),
        Err(e) => return Err(e),
    }
}
