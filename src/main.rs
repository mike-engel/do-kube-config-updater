mod file_structure;

use dirs::home_dir;
use file_structure::KubeConfig;
use log::{debug, info};
use reqwest::Client;
use serde_yaml;
use simplelog::{LevelFilter, TermLogger};
use std::env::var;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
pub(crate) enum ToolError {
    BadGateway(Option<String>),
    ParseError(Option<String>),
    FileError(Option<String>),
}

fn download_config() -> Result<String, ToolError> {
    let client = Client::new();
    let cluster_id = var("CLUSTER_ID").unwrap();
    let auth_token = var("DO_API_KEY").unwrap();
    let url = format!(
        "https://api.digitalocean.com/v2/kubernetes/clusters/{}/kubeconfig",
        cluster_id
    );
    let result = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .and_then(|mut res| res.text());

    match result {
        Ok(config) => Ok(config),
        Err(err) => {
            debug!("Error downloading the config: {:?}", err);

            Err(ToolError::BadGateway(Some(format!(
                "Error downloading the config: {:?}",
                err
            ))))
        }
    }
}

fn get_current_config() -> Result<Option<String>, ToolError> {
    let home = home_dir().unwrap();
    let kube_config_path = home.join(Path::new(".kube/config"));
    let mut contents = String::new();
    let file = File::open(kube_config_path);

    if let Err(err) = file {
        debug!("Error opening the current config file: {:?}", err);

        return Ok(None);
    }

    let read_result = file.unwrap().read_to_string(&mut contents);

    if let Err(err) = read_result {
        debug!("Error reading the current config: {:?}", err);

        return Err(ToolError::FileError(Some(format!(
            "Error reading the current config: {:?}",
            err
        ))));
    }

    Ok(Some(contents))
}

fn save_current_config(config: &KubeConfig) -> Result<(), ToolError> {
    let home = home_dir().unwrap();
    let kube_config_path = home.join(Path::new(".kube/config"));
    let config_str = serde_yaml::to_string(config).unwrap();
    let mut file = File::create(kube_config_path).unwrap();
    let write_result = file.write(config_str.as_bytes());

    match write_result {
        Ok(_) => Ok(()),
        Err(err) => {
            debug!("Error writing the new config: {:?}", err);

            Err(ToolError::FileError(Some(format!(
                "Error writing the new config: {:?}",
                err
            ))))
        }
    }
}

fn do_config_present(current_config: &KubeConfig, new_config: &KubeConfig) -> bool {
    let cluster_name = &new_config.clusters[0].name;

    for cluster in &current_config.clusters {
        if &cluster.name == cluster_name {
            return true;
        }
    }

    false
}

fn update_config(current_config: KubeConfig, new_config: KubeConfig) -> KubeConfig {
    let mut config = KubeConfig { ..current_config };

    for mut user in config.users {
        if &user.name == &new_config.users[0].name {
            user.user.client_certificate_data =
                new_config.users[0].user.client_certificate_data.clone();
            user.user.client_key_data = new_config.users[0].user.client_key_data.clone();
        }
    }

    new_config
}

fn add_config(current_config: KubeConfig, new_config: KubeConfig) -> KubeConfig {
    let mut config = KubeConfig { ..current_config };

    config.clusters.extend(new_config.clusters.clone());
    config.contexts.extend(new_config.contexts.clone());
    config.users.extend(new_config.users.clone());

    config
}

fn parse_config(config: &str) -> Result<KubeConfig, ToolError> {
    match serde_yaml::from_str::<KubeConfig>(config) {
        Ok(value) => Ok(value),
        Err(err) => {
            debug!("Error parsing the config as a valid k8s config: {:?}", err);

            Err(ToolError::ParseError(Some(format!(
                "Error parsing the config as a valid k8s config: {:?}",
                err
            ))))
        }
    }
}

fn main() -> Result<(), ToolError> {
    match TermLogger::init(LevelFilter::Debug, simplelog::Config::default()) {
        Ok(_) => {}
        Err(error) => panic!("Could not set up TermLogger {:?}", error),
    };

    let raw_new_config = download_config()?;
    let new_config = parse_config(&raw_new_config)?;
    let raw_current_config = get_current_config()?;
    let config_to_save = match raw_current_config {
        Some(config) => {
            let current_config = parse_config(&config)?;

            match do_config_present(&current_config, &new_config) {
                true => update_config(current_config, new_config),
                false => add_config(current_config, new_config),
            }
        }
        None => new_config,
    };

    save_current_config(&config_to_save)?;

    info!("New config saved!");

    Ok(())
}
