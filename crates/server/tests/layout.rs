use axum::body::Body;
use axum::http::{Request, StatusCode};
use freemax_server::{AppState, router};
use http_body_util::BodyExt;
use sqlx::PgPool;
use tower::ServiceExt;

/// Ответ на `GET path` без обращения к БД: пул подключается лениво.
async fn get(path: &str, headers: &[(&str, &str)]) -> (StatusCode, String) {
    let db = PgPool::connect_lazy("postgres://localhost/unused").unwrap();
    let mut request = Request::get(path);
    for (name, value) in headers {
        request = request.header(*name, *value);
    }

    let response = router(AppState { db }, "target/web")
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

const HTMX: (&str, &str) = ("HX-Request", "true");

#[tokio::test]
async fn full_page_in_layout() {
    let (status, html) = get("/", &[]).await;

    assert_eq!(status, StatusCode::OK);
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains(r#"<html lang="ru">"#));
    assert!(html.contains("<title>Объявления — Freemax</title>"));
    assert!(html.contains(r#"href="/webassets/main.css""#));
    assert!(html.contains(r#"class="main-content""#));
    assert!(html.contains(r#"id="main-nav""#));
    assert!(!html.contains("hx-swap-oob"));
    assert!(html.contains("color-scheme-selector"));
    assert!(html.contains("Как в системе"));
    assert!(html.contains(r#"aria-label="Цветовая схема""#));
}

#[tokio::test]
async fn htmx_request_gets_fragment() {
    let (status, html) = get("/circles", &[HTMX]).await;

    assert_eq!(status, StatusCode::OK);
    assert!(!html.contains("<html"));
    assert!(!html.contains(r#"class="main-content""#));
    assert!(html.starts_with("<title>Круги — Freemax</title>"));
    assert!(html.contains(r#"<nav id="main-nav" class="page-menu-nav" hx-swap-oob="true">"#));
    assert!(html.contains(r#"href="/circles" hx-get="/circles""#));
    assert!(html.contains(r#"aria-current="page""#));
}

#[tokio::test]
async fn history_restore_gets_full_page() {
    let (_, html) = get("/about", &[HTMX, ("HX-History-Restore-Request", "true")]).await;

    assert!(html.starts_with("<!DOCTYPE html>"));
}

#[tokio::test]
async fn not_found_in_layout() {
    let (status, html) = get("/no-such-page", &[]).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("Страница не найдена"));

    let (status, html) = get("/no-such-page", &[HTMX]).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(html.starts_with("<title>Страница не найдена — Freemax</title>"));
}

#[tokio::test]
async fn language_from_accept_language() {
    let (_, html) = get("/", &[("Accept-Language", "en-US,en;q=0.9,ru;q=0.5")]).await;

    assert!(html.contains(r#"<html lang="en">"#));
    assert!(html.contains("<title>Listings — Freemax</title>"));
    assert!(html.contains("<span>Circles</span>"));
    assert!(html.contains("Freemax — free exchange"));
    assert!(html.contains(">Dark<"));
}

#[tokio::test]
async fn language_cookie_over_accept_language() {
    let (_, html) = get("/about", &[("Cookie", "lang=ru"), ("Accept-Language", "en")]).await;
    assert!(html.contains(r#"<html lang="ru">"#));
    assert!(html.contains("<title>О проекте — Freemax</title>"));

    let (_, html) = get("/circles", &[HTMX, ("Cookie", "lang=en")]).await;
    assert!(html.starts_with("<title>Circles — Freemax</title>"));
}

#[tokio::test]
async fn error_page_in_request_language() {
    let (status, html) = get("/no-such-page", &[("Cookie", "lang=en")]).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(html.contains("<title>Page not found — Freemax</title>"));
    assert!(html.contains("Go to the home page"));
}

#[tokio::test]
async fn response_varies_by_language() {
    let db = PgPool::connect_lazy("postgres://localhost/unused").unwrap();
    let response = router(AppState { db }, "target/web")
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let vary = response.headers()["vary"].to_str().unwrap();
    assert!(vary.contains("cookie") && vary.contains("accept-language"));
}
