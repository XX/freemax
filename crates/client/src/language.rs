//! Переключатель языка в шапке страницы. Разметку рендерит сервер (`freemax_server::view::language`).

use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Event, HtmlDocument};
use wingy_hypertext_web::util::event;

/// Должен совпадать с `SELECTOR_ID` в `freemax_server::view::language`.
const SELECTOR_ID: &str = "language-selector";

/// Должен совпадать с `LANG_COOKIE` в `freemax_server::i18n`.
const LANG_COOKIE: &str = "lang";

/// Ключ кэша истории htmx в `localStorage`: снимки страниц на прежнем языке после смены не нужны.
const HTMX_HISTORY_CACHE: &str = "htmx-history-cache";

/// При выборе языка сохраняет его в cookie и перезагружает страницу: язык влияет на весь layout.
pub fn listen_language_selector() {
    dom::existing::document().add_steady_event_listener(event::SELECT, |event| {
        handle_select(&event);
    });
}

fn handle_select(event: &Event) -> Option<()> {
    let dropdown = event.target()?.maybe_into_element()?;
    if dropdown.id() != SELECTOR_ID {
        return None;
    }

    let custom: &CustomEvent = event.dyn_ref()?;
    let lang = custom.detail().get("value").as_string()?;
    if dom::existing::document_element().get_attribute("lang").as_deref() == Some(lang.as_str()) {
        return None;
    }

    let document: HtmlDocument = dom::existing::document().dyn_into().ok()?;
    let cookie = format!("{LANG_COOKIE}={lang}; path=/; max-age=31536000; samesite=lax");
    document.set_cookie(&cookie).ok()?;
    if let Some(storage) = dom::existing::local_storage() {
        storage.remove_item(HTMX_HISTORY_CACHE).ok();
    }
    dom::existing::location().reload().ok()
}
