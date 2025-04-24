use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{ Arc, RwLock };
use chrono::Local;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use tokio;

use super::locales::Language;
use super::messages::Messages;

lazy_static! {
    pub(crate) static ref MESSAGES_CACHE: RwLock<HashMap<String, Arc<RwLock<Messages>>>> = {
        RwLock::new(HashMap::new())
    };
    pub(crate) static ref RUNNER: tokio::runtime::Runtime = {
        tokio::runtime::Runtime::new().unwrap()
    };
}

pub fn get_cached_messages_test() -> &'static RwLock<HashMap<String, Arc<RwLock<Messages>>>> {
    return &*MESSAGES_CACHE;
}

pub fn get_messages<E>(bundle_name: Arc<str>, lang: Language) -> Option<Arc<RwLock<Messages>>>
    where E: Eq + Hash + std::str::FromStr + std::fmt::Display + IntoEnumIterator
{
    tokio::spawn(remove_expired_messages());
    let mut cache = MESSAGES_CACHE.write().unwrap();
    if let Some(existing) = cache.get(bundle_name.as_ref()) {
        let mut writtable = existing.write().unwrap();
        if !writtable.lang.eq(&lang) {
            writtable.change_locale_and_validate::<E>(lang);
        }
        return Some(Arc::clone(existing));
    }
    return Some(
        Arc::clone(
            cache
                .entry(bundle_name.to_string())
                .or_insert_with(|| {
                    Arc::new(RwLock::new(Messages::new::<E>(bundle_name.as_ref(), lang)))
                })
        )
    );
}

pub async fn remove_expired_messages() {
    let now = Local::now();
    let mut cache = MESSAGES_CACHE.write().unwrap();
    cache.retain(|_, msg| { msg.read().unwrap().duration >= now });
}
