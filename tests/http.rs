use scafra::web::{
    axum::{
        body::to_bytes,
        body::Body,
        http::{Request, StatusCode},
    },
    tower::util::ServiceExt,
};
use scafra_test_service as _;

#[tokio::test]
async fn petstore_supports_crud_lifecycle() {
    let app = scafra::build_router().expect("petstore routes should build");
    let response = app
        .clone()
        .oneshot(json_request(
            Request::post("/pet"),
            r#"{"name":"Milo","photoUrls":["https://example.test/milo"],"tags":["cat"],"status":"available"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(Request::get("/pet/1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    assert!(body.windows(b"Milo".len()).any(|window| window == b"Milo"));

    let response = app
        .clone()
        .oneshot(json_request(
            Request::put("/pet/1"),
            r#"{"name":"Milo updated","tags":["cat","friendly"],"status":"sold"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(Request::delete("/pet/1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = app
        .oneshot(Request::get("/pet/1").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn petstore_validates_json_and_pet_names() {
    let app = scafra::build_router().expect("petstore routes should build");
    let response = app
        .clone()
        .oneshot(json_request(Request::post("/pet"), r#"{"name":""}"#))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = app
        .oneshot(json_request(Request::post("/pet"), r#"{"name":"Milo""#))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn petstore_filters_by_status_and_tags() {
    let app = scafra::build_router().expect("petstore routes should build");
    for payload in [
        r#"{"name":"Milo","tags":["cat"],"status":"available"}"#,
        r#"{"name":"Rex","tags":["dog"],"status":"sold"}"#,
    ] {
        let response = app
            .clone()
            .oneshot(json_request(Request::post("/pet"), payload))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }

    let response = app
        .clone()
        .oneshot(
            Request::get("/pet/findByStatus?status=available")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    assert!(body.windows(b"Milo".len()).any(|window| window == b"Milo"));
    assert!(!body.windows(b"Rex".len()).any(|window| window == b"Rex"));

    let response = app
        .oneshot(
            Request::get("/pet/findByTags?tags=dog")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    assert!(body.windows(b"Rex".len()).any(|window| window == b"Rex"));
}

fn json_request(builder: scafra::web::axum::http::request::Builder, body: &str) -> Request<Body> {
    builder
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap()
}
