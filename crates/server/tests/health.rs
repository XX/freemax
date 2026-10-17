use axum::body::Body;
use axum::http::{Request, StatusCode};
use freemax_server::{AppState, router};
use http_body_util::BodyExt;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(migrator = "freemax_server::MIGRATOR")]
async fn health_ok(db: PgPool) {
    let response = router(AppState { db })
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"ok");
}

#[sqlx::test(migrator = "freemax_server::MIGRATOR")]
async fn migrations_applied(db: PgPool) {
    let tables: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public' \
         AND table_name IN ('participants', 'circles', 'memberships', 'listings', 'listing_circles', 'responses')",
    )
    .fetch_one(&db)
    .await
    .unwrap();

    assert_eq!(tables, 6);
}
