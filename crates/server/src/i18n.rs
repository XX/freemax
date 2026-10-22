//! Локализация интерфейса: Fluent-ресурсы из `locales/<язык>/*.ftl`, встроенные в бинарник.

use std::convert::Infallible;

use axum::extract::{FromRequestParts, Request};
use axum::http::HeaderMap;
use axum::http::header::{ACCEPT_LANGUAGE, COOKIE};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use fluent_templates::{LanguageIdentifier, Loader, langid, static_loader};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "ru",
        // Без изолирующих символов Unicode вокруг подстановок: языки только с письмом слева направо.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

/// Cookie с выбранным языком. Должен совпадать с `LANG_COOKIE` в `freemax_client::language`.
pub const LANG_COOKIE: &str = "lang";

static RU: LanguageIdentifier = langid!("ru");
static EN: LanguageIdentifier = langid!("en");

/// Язык интерфейса.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Locale {
    #[default]
    Ru,
    En,
}

impl Locale {
    pub const ALL: [Self; 2] = [Self::Ru, Self::En];

    /// Код языка для `<html lang>` и cookie.
    pub fn code(self) -> &'static str {
        match self {
            Self::Ru => "ru",
            Self::En => "en",
        }
    }

    /// Язык по тегу BCP 47: учитывается только основной подтег (`en-US` → `en`).
    pub fn parse(tag: &str) -> Option<Self> {
        let primary = tag.trim().split(['-', '_']).next()?;
        Self::ALL
            .into_iter()
            .find(|locale| locale.code().eq_ignore_ascii_case(primary))
    }

    /// Язык запроса: cookie [`LANG_COOKIE`], затем `Accept-Language`, иначе язык по умолчанию.
    pub fn from_headers(headers: &HeaderMap) -> Self {
        headers
            .get_all(COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .find_map(|cookies| cookie_value(cookies, LANG_COOKIE).and_then(Self::parse))
            .or_else(|| {
                let header = headers.get(ACCEPT_LANGUAGE)?.to_str().ok()?;
                negotiate(header)
            })
            .unwrap_or_default()
    }

    fn langid(self) -> &'static LanguageIdentifier {
        match self {
            Self::Ru => &RU,
            Self::En => &EN,
        }
    }

    /// Сообщение `id`. Если его нет ни в этом языке, ни в языке по умолчанию, — ошибка в лог и сам `id`.
    pub fn t(self, id: &str) -> String {
        LOCALES.try_lookup(self.langid(), id).unwrap_or_else(|| {
            tracing::error!("missing localization message {id:?} for {}", self.code());
            id.to_owned()
        })
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Locale {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(parts
            .extensions
            .get::<Self>()
            .copied()
            .unwrap_or_else(|| Self::from_headers(&parts.headers)))
    }
}

/// Middleware: определяет язык запроса и кладёт [`Locale`] в расширения запроса.
pub async fn detect_locale(mut request: Request, next: Next) -> Response {
    let locale = Locale::from_headers(request.headers());
    request.extensions_mut().insert(locale);
    next.run(request).await
}

/// Значение cookie `name` из заголовка `Cookie`.
fn cookie_value<'a>(cookies: &'a str, name: &str) -> Option<&'a str> {
    cookies.split(';').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key.trim() == name).then(|| value.trim())
    })
}

/// Поддерживаемый язык с наибольшим весом `q` из `Accept-Language`; при равных весах — первый в списке.
fn negotiate(header: &str) -> Option<Locale> {
    let mut best: Option<(Locale, f32)> = None;
    for item in header.split(',') {
        let mut parts = item.split(';');
        let Some(locale) = parts.next().and_then(Locale::parse) else {
            continue;
        };
        let quality = parts
            .find_map(|param| param.trim().strip_prefix("q="))
            .map_or(Some(1.0), |q| q.trim().parse::<f32>().ok())
            .unwrap_or(0.0);
        if quality > 0.0 && best.is_none_or(|(_, best_quality)| quality > best_quality) {
            best = Some((locale, quality));
        }
    }
    best.map(|(locale, _)| locale)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use axum::http::HeaderValue;
    use fluent_syntax::ast::Entry;

    use super::*;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (name, value) in pairs {
            headers.append(*name, HeaderValue::from_static(value));
        }
        headers
    }

    #[test]
    fn parse_tag() {
        assert_eq!(Locale::parse("ru"), Some(Locale::Ru));
        assert_eq!(Locale::parse("en-US"), Some(Locale::En));
        assert_eq!(Locale::parse(" EN "), Some(Locale::En));
        assert_eq!(Locale::parse("de"), None);
        assert_eq!(Locale::parse(""), None);
    }

    #[test]
    fn negotiate_accept_language() {
        assert_eq!(negotiate("en-US,en;q=0.9,ru;q=0.8"), Some(Locale::En));
        assert_eq!(negotiate("de-DE,de;q=0.9,en;q=0.5,ru;q=0.7"), Some(Locale::Ru));
        assert_eq!(negotiate("ru;q=0, en;q=0.1"), Some(Locale::En));
        assert_eq!(negotiate("de, *;q=0.5"), None);
        assert_eq!(negotiate(""), None);
    }

    #[test]
    fn cookie_has_priority() {
        let detect = |pairs| Locale::from_headers(&headers(pairs));

        assert_eq!(detect(&[]), Locale::Ru);
        assert_eq!(detect(&[("accept-language", "en")]), Locale::En);
        assert_eq!(
            detect(&[("cookie", "a=1; lang=ru"), ("accept-language", "en")]),
            Locale::Ru
        );
        assert_eq!(detect(&[("cookie", "lang=xx"), ("accept-language", "en")]), Locale::En);
        assert_eq!(detect(&[("cookie", "a=1"), ("cookie", "lang=en")]), Locale::En);
    }

    #[test]
    fn lookup() {
        assert_eq!(Locale::Ru.t("nav-about"), "О проекте");
        assert_eq!(Locale::En.t("nav-about"), "About");
        assert_eq!(Locale::En.t("no-such-message"), "no-such-message");
    }

    /// Идентификаторы сообщений и их атрибутов из всех `.ftl` языка.
    fn message_ids(locale: Locale) -> BTreeSet<String> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("locales")
            .join(locale.code());
        let mut ids = BTreeSet::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "ftl") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            let resource = fluent_syntax::parser::parse(source.as_str())
                .unwrap_or_else(|(_, errors)| panic!("{}: {errors:?}", path.display()));
            for entry in resource.body {
                let (prefix, id, attributes) = match entry {
                    Entry::Message(message) => ("", message.id, message.attributes),
                    Entry::Term(term) => ("-", term.id, term.attributes),
                    _ => continue,
                };
                let id = format!("{prefix}{}", id.name);
                for attribute in attributes {
                    ids.insert(format!("{id}.{}", attribute.id.name));
                }
                assert!(ids.insert(id.clone()), "{}: duplicate {id}", path.display());
            }
        }
        ids
    }

    #[test]
    fn locales_are_complete() {
        let default = message_ids(Locale::default());
        for locale in Locale::ALL {
            let ids = message_ids(locale);
            let missing: Vec<_> = default.difference(&ids).collect();
            let extra: Vec<_> = ids.difference(&default).collect();
            assert!(missing.is_empty(), "{}: missing {missing:?}", locale.code());
            assert!(extra.is_empty(), "{}: not in default locale {extra:?}", locale.code());
        }
    }
}
