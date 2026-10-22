//! Содержимое страниц.

use axum::http::StatusCode;
use hypertext::prelude::*;
use iconic::fontawesome;
use wingy_hypertext::class::{HEADING_L, ICON, STACK};

use super::View;
use crate::i18n::Locale;

pub fn index(locale: Locale) -> View {
    let title = locale.t("listings-title");
    View::new(title.clone(), rsx! {
        <div class=STACK>
            <h1 class=HEADING_L>(title)</h1>
            <p>(locale.t("listings-placeholder"))</p>
        </div>
    })
}

pub fn circles(locale: Locale) -> View {
    let title = locale.t("circles-title");
    View::new(title.clone(), rsx! {
        <div class=STACK>
            <h1 class=HEADING_L>(title)</h1>
            <p>(locale.t("circles-placeholder"))</p>
        </div>
    })
}

pub fn about(locale: Locale) -> View {
    let title = locale.t("about-title");
    View::new(title.clone(), rsx! {
        <div class=STACK>
            <h1 class=HEADING_L>(title)</h1>
            <p>(locale.t("about-intro"))</p>
            <ul>
                <li>(locale.t("about-free"))</li>
                <li>(locale.t("about-circles"))</li>
                <li>(locale.t("about-joining"))</li>
            </ul>
        </div>
    })
}

/// Страница ошибки; без `message` — стандартное сообщение для статуса.
pub fn error(locale: Locale, status: StatusCode, message: Option<&str>) -> View {
    let (title_id, message_id) = match status {
        StatusCode::NOT_FOUND => ("error-not-found", "error-not-found-message"),
        StatusCode::FORBIDDEN => ("error-forbidden", "error-forbidden-message"),
        StatusCode::BAD_REQUEST => ("error-bad-request", "error-internal-message"),
        _ => ("error-internal", "error-internal-message"),
    };
    let title = locale.t(title_id);
    let message = message.map_or_else(|| locale.t(message_id), str::to_owned);

    View::new(title.clone(), rsx! {
        <div class=STACK>
            <h1 class=HEADING_L>
                <span class=ICON>(fontawesome::solid::TriangleExclamation)</span>
                " " (title)
            </h1>
            <p>(message)</p>
            <p><a href="/">(locale.t("error-home-link"))</a></p>
        </div>
    })
}
