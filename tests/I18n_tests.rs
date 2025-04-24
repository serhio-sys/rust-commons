use std::{ sync::Arc, time::Duration };

use rust_commons::{
    i18n::{
        locales::Language,
        messages_cache::{ get_cached_messages_test, get_messages, remove_expired_messages },
    },
    logger,
};
use strum_macros::{ Display, EnumIter, EnumString };
use tokio::time;

const LOCALE_TEST_FOLDER: &str = "tests/test_locale";
const TEST_MESSAGES: &str = "auth_service_messages";

fn setup() {
    logger::init_logger();
    std::env::set_var("EXPIRE_INTERVAL", "1");
    std::env::set_var("LOCALE_PATH", LOCALE_TEST_FOLDER);
    get_cached_messages_test().write().unwrap().clear();
}

#[derive(EnumString, Display, PartialEq, Eq, Hash, EnumIter)]
pub enum TestMessages {
    #[strum(serialize = "hello")] Hello,
    #[strum(serialize = "home")] Home,
}

#[tokio::test]
async fn cache_read_messages_test() {
    setup();
    let binding = get_messages::<TestMessages>(
        TEST_MESSAGES.into(),
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
        TEST_MESSAGES.into(),
        rust_commons::i18n::locales::Language::English
    ).unwrap();
    let message = binding.read().unwrap();
    let expected: Arc<str> = Arc::from("Hello Guys");
    assert_eq!(expected.as_ref(), message.get_message_by_enum(TestMessages::Hello).unwrap());
    assert_eq!("hello_test", message.get_message_by_enum(TestMessages::Home).unwrap());
}

#[tokio::test]
async fn clean_up_cache_test() {
    setup();
    let _ = get_messages::<TestMessages>(
        TEST_MESSAGES.into(),
        rust_commons::i18n::locales::Language::English
    );

    assert_eq!(1, get_cached_messages_test().read().unwrap().len());
    time::sleep(Duration::from_millis(1000)).await;
    remove_expired_messages().await;
    assert_eq!(0, get_cached_messages_test().read().unwrap().len());
}
