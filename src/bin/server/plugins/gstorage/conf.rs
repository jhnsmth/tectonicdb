use plugins::gstorage::config;
use std::collections::HashMap;
use std::default::Default;

#[cfg(test)]
static GSTORAGE_CONF_FNAME: &str = "conf/gstorage/example.conf.toml";
#[cfg(not(test))]
static GSTORAGE_CONF_FNAME: &str = "conf/gstorage/conf";

pub struct GStorageConfig {
    pub conf: HashMap<String, String>,
    pub oauth_token: Option<String>,
    pub bucket_name: String,
    pub folder: String,
    pub interval: u64,
}

impl GStorageConfig {

    pub fn new() -> GStorageConfig {
        let conf = GStorageConfig::get_conf();
        let oauth_token = {
            if conf.contains_key("oauth") {
                Some(conf.get("oauth").unwrap()
                         .to_owned())
            } else {
                None
            }
        };

        let bucket_name = conf.get("bucket-name").unwrap()
            .to_owned();
        let folder = conf.get("folder").unwrap_or(&"".to_owned())
            .to_owned();

        // upload interval
        let interval = conf.get("interval")
            .unwrap_or(&"3600".to_owned()).to_owned();
        

        GStorageConfig {
            conf,
            oauth_token,
            bucket_name,
            folder,
            interval: interval.parse().unwrap(),
        }

    }

    fn get_conf() -> HashMap<String, String> {
        let mut settings = config::Config::default();
        settings.merge(config::File::with_name(GSTORAGE_CONF_FNAME)).unwrap();
        settings.deserialize::<HashMap<String, String>>().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_return_config() {
        let config = GStorageConfig::new();
    }
}