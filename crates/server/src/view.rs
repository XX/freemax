//! Представления: общий layout страницы и частичный рендер для htmx.

use std::borrow::Cow;

use axum::body::Body;
use axum::extract::Request;
use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE, VARY};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use hypertext::prelude::*;
use hypertext::{Raw, Rendered};

use crate::i18n::Locale;

pub mod color_scheme;
pub mod language;
pub mod layout;
pub mod page;

/// Заголовок, который htmx добавляет к своим запросам.
pub const HX_REQUEST: &str = "hx-request";

/// Заголовок запроса htmx на восстановление страницы из истории, когда её нет в кэше: нужна полная страница.
pub const HX_HISTORY_RESTORE_REQUEST: &str = "hx-history-restore-request";

/// Содержимое основной области страницы с заголовком.
///
/// Обработчик возвращает только содержимое; во что его обернуть — в полную страницу или во фрагмент для htmx —
/// решает [`render_layout`], которому известен запрос.
#[derive(Clone)]
pub struct View {
    pub title: Cow<'static, str>,
    pub content: Raw<String>,
}

impl View {
    pub fn new(title: impl Into<Cow<'static, str>>, content: impl Renderable) -> Self {
        // XSS SAFETY: строка получена рендерингом `Renderable`, значения в ней уже экранированы.
        let content = Raw::dangerously_create(content.render().into_inner());
        Self {
            title: title.into(),
            content,
        }
    }
}

impl IntoResponse for View {
    fn into_response(self) -> Response {
        let mut response = StatusCode::OK.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

/// Страница ошибки. Обработчику язык может быть неизвестен ([`crate::AppError`]), поэтому текст подставляет
/// [`render_layout`] на языке запроса.
#[derive(Clone)]
pub struct ErrorView {
    /// Сообщение для пользователя; если нет — стандартное для статуса.
    pub message: Option<String>,
}

impl IntoResponse for ErrorView {
    fn into_response(self) -> Response {
        let mut response = StatusCode::OK.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

/// Нужен ли только фрагмент вместо полной страницы.
pub fn is_partial(headers: &HeaderMap) -> bool {
    let is_true = |name| headers.get(name).is_some_and(|value| value == "true");
    is_true(HX_REQUEST) && !is_true(HX_HISTORY_RESTORE_REQUEST)
}

/// Middleware: оборачивает [`View`] или [`ErrorView`] из ответа в полную страницу или во фрагмент для htmx.
///
/// Язык берётся из [`Locale`] в расширениях запроса, поэтому [`crate::i18n::detect_locale`] должен стоять снаружи.
pub async fn render_layout(request: Request, next: Next) -> Response {
    let partial = is_partial(request.headers());
    let path = request.uri().path().to_owned();
    let locale = request.extensions().get::<Locale>().copied().unwrap_or_default();

    let mut response = next.run(request).await;
    let extensions = response.extensions_mut();
    let view = match (extensions.remove::<View>(), extensions.remove::<ErrorView>()) {
        (Some(view), _) => view,
        (None, Some(error)) => page::error(locale, response.status(), error.message.as_deref()),
        (None, None) => return response,
    };

    let html: Rendered<String> = if partial {
        layout::fragment(&view, &path, locale).render()
    } else {
        layout::document(&view, &path, locale).render()
    };

    let headers = response.headers_mut();
    headers.remove(CONTENT_LENGTH);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8"));
    // Ответ зависит от htmx-заголовка и языка (cookie или `Accept-Language`).
    headers.insert(VARY, HeaderValue::from_static("hx-request, cookie, accept-language"));
    *response.body_mut() = Body::from(html.into_inner());
    response
}
