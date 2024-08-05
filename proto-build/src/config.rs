use std::{collections::HashMap, env};

use config::Config as ConfigBuilder;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub projects: HashMap<String, Project>,
}

//#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Project {
    pub path: String,
    pub target_dir: Vec<String>,
}

impl ProjectConfig {
    pub fn from_yaml(path: &str) -> Self {
        let settings = ConfigBuilder::builder()
            .add_source(config::File::with_name(
                format!("{}/config.yaml", path).as_str(),
            ))
            .build()
            .expect("config.yaml should not fail");
        settings
            .try_deserialize::<Self>()
            .expect("config.yaml deserialize should not fail")
    }
}

#[allow(dead_code)]
pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path
            .into_os_string()
            .into_string()
            .expect("Current directory. should not fail"),
        Err(_) => "FAILED".to_string(),
    }
}

mod test {

    #[test]
    fn test_config() {
        crate::proto::ProtoConfig::from_yaml("../config");
    }
}
