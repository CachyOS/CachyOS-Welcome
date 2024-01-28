use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
use i18n_embed::{DefaultLocalizer, I18nEmbedError, LanguageLoader, Localizer};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localization::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localization::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

/// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

/// Get translated text
pub fn get_locale_text(message_id: &str) -> String {
    LANGUAGE_LOADER.get(message_id)
}

/// Get available languages
pub fn get_available_languages() -> Result<Vec<unic_langid::LanguageIdentifier>, I18nEmbedError> {
    LANGUAGE_LOADER.available_languages(&Localizations)
}

/// Check if language is available
pub fn check_language_valid(requested_language: &str) -> bool {
    let requested_lang_id: LanguageIdentifier =
        requested_language.parse().expect("Invalid locale id");

    let available_languages = get_available_languages().unwrap();
    available_languages.iter().any(|x| *x == requested_lang_id)
}

/// Get system language or default language if system one is not supported
///
/// Checks env variables in following order:
/// - `LC_ALL`
/// - `LC_MESSAGES`
/// - `LANG`
pub fn get_default_lang() -> String {
    std::env::var("LC_ALL").unwrap_or_else(|_| {
        std::env::var("LC_MESSAGES").unwrap_or_else(|_| {
            std::env::var("LANG").unwrap_or_else(|_| String::from("en_US.UTF-8"))
        })
    })
}
