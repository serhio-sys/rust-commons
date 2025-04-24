use std::sync::Arc;

use rust_commons::{
    i18n::{
        locales::Language,
        messages::EnumMessages,
        messages_cache::{ get_cached_messages_test, get_messages },
    },
    logger,
};
use strum_macros::{ Display, EnumIter, EnumString };

const LOCALE_TEST_FOLDER: &str = "tests/test_locale";
const TEST_MESSAGES: &str = "auth_service_messages";

fn setup() {
    logger::init_logger();
    std::env::set_var("EXPIRE_INTERVAL", "1");
    std::env::set_var("LOCALE_PATH", LOCALE_TEST_FOLDER);
    if
        let Err(e) = get_cached_messages_test()
            .write()
            .map(|mut cache| cache.clear())
    {
        eprintln!("Failed to clear cached messages: {}", e);
    } else {
        println!("Cached messages cleared successfully.");
    }
}

#[derive(EnumString, Display, PartialEq, Eq, Hash, EnumIter)]
pub enum TestMessages {
    #[strum(serialize = "hello")] Hello,
    #[strum(serialize = "home")] Home,
}

impl EnumMessages for TestMessages {
    fn get_bundle_name() -> String {
        TEST_MESSAGES.to_string()
    }
}

#[tokio::test]
async fn cache_read_messages_test() {
    setup();
    let binding = get_messages::<TestMessages>(
        rust_commons::i18n::locales::Language::English
    ).unwrap();
    let mut message = binding.write().unwrap();
    assert_eq!(1, get_cached_messages_test().read().unwrap().len());
    let expected: Arc<str> = Arc::from("Hello Guys");
    assert_eq!(
        expected.as_ref(),
        message.get_message_by_enum::<TestMessages>(TestMessages::Hello).unwrap()
    );
    message.change_locale(Language::German);
    let expected: Arc<str> = Arc::from("Guten Tag Zusammen");
    assert_eq!(
        expected.as_ref(),
        message.get_message_by_enum::<TestMessages>(TestMessages::Hello).unwrap()
    );
}

#[tokio::test]
async fn message_not_found_test() {
    setup();
    let binding = get_messages::<TestMessages>(
        rust_commons::i18n::locales::Language::English
    ).unwrap();
    let message = binding.read().unwrap();
    let expected: Arc<str> = Arc::from("Hello Guys");
    assert_eq!(expected.as_ref(), message.get_message_by_enum(TestMessages::Hello).unwrap());
    assert_eq!("home", message.get_message_by_enum(TestMessages::Home).unwrap());
}
