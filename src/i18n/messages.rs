use std::{ collections::HashMap, env, fs, hash::Hash, path::PathBuf, time::Duration };

use strum::IntoEnumIterator;
use chrono::{ DateTime, Local };
use dotenvy::dotenv;
use lazy_static::lazy_static;

use crate::config::get_var_or_default;

use super::locales::Language;

const PATH_VARIABLE_NAME: &str = "LOCALE_PATH";
const BASE_LOCALE_PATH: &str = "locale";
const ENV_VARIABLE_KEY: &str = "EXPIRE_INTERVAL";
const DEFAULT_MESSAGE_REMOVE_INTERVAL: u32 = 150;

lazy_static! {
    pub(crate) static ref LOCALE_PATH: String = {
        if let Err(exc) = dotenv() {
            log::warn!(
                "Not found .env file - [{}]. Will used automatically default values",
                exc.to_string()
            );
        }
        let value = get_var_or_default(PATH_VARIABLE_NAME, BASE_LOCALE_PATH);
        return value;
    };
    pub(crate) static ref REMOVE_INTERVAL: u64 = {
        if let Err(exc) = dotenv() {
            log::warn!(
                "Not found .env file - [{}]. Will used automatically default values",
                exc.to_string()
            );
        }
        let value = get_var_or_default(
            ENV_VARIABLE_KEY,
            DEFAULT_MESSAGE_REMOVE_INTERVAL.to_string().as_str()
        );
        match value.parse::<u64>() {
            Ok(num) => {
                return num;
            }
            Err(e) => panic!("Failed to parse: {}", e),
        }
    };
}

pub trait EnumMessages: Eq + Hash + std::fmt::Display + IntoEnumIterator {
    fn get_bundle_name() -> String;
}

pub struct Messages {
    pub bundle_path: String,
    pub messages: HashMap<String, String>,
    pub lang: Language,
    pub(crate) duration: DateTime<Local>,
}

impl Messages {
    pub fn get_message(&self, key: &str) -> Option<String> {
        let message: Option<&String> = self.messages.get(key);
        if message.is_none() {
            log::warn!("Message was not found by key - {}", key);
            return Some(key.to_string());
        }
        message.cloned()
    }

    pub fn get_message_by_enum<E>(&self, key: E) -> Option<String> where E: EnumMessages {
        let key_string = key.to_string();
        let message: Option<&String> = self.messages.get(&key_string);
        if message.is_none() {
            log::warn!("Message was not found by key - {}", &key_string);
            return Some(key_string);
        }
        message.cloned()
    }

    pub fn change_locale(&mut self, lang: Language) {
        let mut path = env::current_dir().unwrap();
        path = path.join(LOCALE_PATH.as_str());
        path = path.join(format!("{}_{}.json", self.bundle_path, lang));
        match Messages::simple_read_messages(&path) {
            Ok(refetched_messages) => {
                self.messages = refetched_messages;
            }
            Err(e) =>
                log::error!(
                    "Error to parse messages. Bundle name - [{}]. \n Trace: {}",
                    self.bundle_path,
                    e
                ),
        }
        self.lang = lang;
    }

    pub fn change_locale_and_validate<E>(&mut self, lang: Language) where E: EnumMessages {
        let mut path = env::current_dir().unwrap();
        path = path.join(LOCALE_PATH.as_str());
        path = path.join(format!("{}_{}.json", self.bundle_path, lang));
        match Messages::read_messages::<E>(&path) {
            Ok(refetched_messages) => {
                self.messages = refetched_messages;
            }
            Err(e) =>
                log::error!(
                    "Error to parse messages. Bundle name - [{}]. \n Trace: {}",
                    self.bundle_path,
                    e
                ),
        }
        self.lang = lang;
    }

    pub fn new<E>(lang: Language) -> Self where E: EnumMessages {
        let bundle_path: String = E::get_bundle_name();
        let mut bundle_messages: HashMap<String, String> = HashMap::new();
        let mut path = env::current_dir().unwrap();
        path = path.join(LOCALE_PATH.as_str());
        path = path.join(format!("{}_{}.json", bundle_path, lang));
        match Messages::read_messages::<E>(&path) {
            Ok(messages) => {
                bundle_messages = messages;
            }
            Err(e) =>
                log::error!(
                    "Error to parse messages. Bundle name - [{}]. \n Trace: {}",
                    bundle_path,
                    e
                ),
        }
        Messages {
            bundle_path: bundle_path.to_owned(),
            messages: bundle_messages,
            lang,
            duration: Local::now() + Duration::from_secs(*REMOVE_INTERVAL),
        }
    }

    fn read_messages<E>(
        path: &PathBuf
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
        where E: EnumMessages
    {
        let map: HashMap<String, String> = Messages::simple_read_messages(path)?;
        for key in E::iter() {
            let key_text = key.to_string();
            let localized_text = map.get(&key_text);
            if localized_text.is_none() {
                log::warn!("Not found message by key: {}", &key_text);
                continue;
            }
        }
        Ok(map)
    }

    fn simple_read_messages(
        path: &PathBuf
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let map: HashMap<String, String> = serde_json::from_str(&file_content)?;
        Ok(map)
    }
}
