use serde::{ Deserialize, Serialize };
use strum_macros::{ Display, EnumIter, EnumString };

#[derive(EnumString, Display, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Language {
    #[strum(serialize = "en")] English,
    #[strum(serialize = "es")] Spanish,
    #[strum(serialize = "fr")] French,
    #[strum(serialize = "de")] German,
    #[strum(serialize = "it")] Italian,
    #[strum(serialize = "pt")] Portuguese,
    #[strum(serialize = "ru")] Russian,
    #[strum(serialize = "zh")] Chinese,
    #[strum(serialize = "ja")] Japanese,
    #[strum(serialize = "ko")] Korean,
    #[strum(serialize = "ar")] Arabic,
    #[strum(serialize = "hi")] Hindi,
    #[strum(serialize = "tr")] Turkish,
    #[strum(serialize = "pl")] Polish,
    #[strum(serialize = "nl")] Dutch,
    #[strum(serialize = "sv")] Swedish,
    #[strum(serialize = "no")] Norwegian,
    #[strum(serialize = "da")] Danish,
    #[strum(serialize = "fi")] Finnish,
}
