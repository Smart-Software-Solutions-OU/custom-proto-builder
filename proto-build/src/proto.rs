use std::env;

use config::{Config as ConfigBuilder, Map};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProtoConfig {
    pub protos: Vec<Proto>,
}

//#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Proto {
    pub repository: String,
    pub tag: String,
    pub proto_folder: String,
    pub merge: bool,
    pub modules: Vec<String>,
    pub black_list: Vec<String>,
    pub custom: Map<String, String>,
}

impl ProtoConfig {
    pub fn from_yaml(path: &str) -> Self {
        let settings = ConfigBuilder::builder()
            .add_source(config::File::with_name(path))
            .build()
            .expect("config.yaml should not fail");
        // if loading the file fail, theres nothing we can do, panic
        settings
            .try_deserialize::<Self>()
            .expect("config.yaml deserialize should not fail")
    }
}

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
