use anyhow::Error;
use std::io::Read;

use once_cell::sync::Lazy;
use yaml_rust::{Yaml, YamlLoader};

pub static CONFIG: Lazy<ApplicationConfig> = Lazy::new(ApplicationConfig::default);

/// Data structure for the config of Mysql Connection
#[derive(Debug)]
pub struct ApplicationConfig {
    pub server_url: String,
    pub log_path: String,
    pub mysql_url: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let mut yml_data = String::new();
        let _ = std::fs::File::open("application.yml")
            .expect("application.yml is not found!")
            .read_to_string(&mut yml_data);

        let docs = YamlLoader::load_from_str(&yml_data).unwrap();

        let server_url = get_cfg(&docs, "server_url").unwrap();
        let log_path = get_cfg(&docs, "log_path").unwrap();
        let mysql_url = get_cfg(&docs, "mysql_url").unwrap();

        Self {
            server_url: server_url.as_str().unwrap_or("").to_string(),
            log_path: log_path.as_str().unwrap_or("").to_string(),
            mysql_url: mysql_url.as_str().unwrap_or("").to_string(),
        }
    }
}

/// get the value config which from a yml type file with a key name.
fn get_cfg<'a>(docs: &'a Vec<Yaml>, key: &str) -> Result<&'a Yaml, Error> {
    for x in docs {
        if let Yaml::Hash(hash) = x {
            let v = hash.get(&Yaml::String(key.to_string()));
            if let Some(v) = v {
                return Ok(v);
            }
        }
    }
    Err(anyhow::anyhow!(format!(
        "in application.yml key: '{}' not exist!",
        key
    )))
}

pub fn test_config() -> anyhow::Result<()> {
    // left
    let config = ApplicationConfig::default();

    // right
    let mut txt = vec![];

    match crate::utils::read_lines("./application.yml") {
        Ok(lines) => {
            for s in lines.flatten() {
                if !s.starts_with('#') {
                    let sp = s.splitn(2, ':');
                    txt.push(
                        sp.last()
                            .map(|ss| ss.trim_matches(|c| c == ' ' || c == '"'))
                            .unwrap()
                            .to_owned(),
                    );
                }
            }
        }
        Err(e) => {
            dbg!("Open File Error : {}", e.to_string());
        }
    }

    assert_eq!(config.server_url, txt[0]);
    assert_eq!(config.log_path, txt[1]);
    assert_eq!(config.mysql_url, txt[2]);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    /// ensure the mysql connection config data structure is correctly match the yml file.
    #[test]
    fn test_config_warp() {
        test_config().unwrap();
    }
}
