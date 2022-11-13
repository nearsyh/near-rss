use crate::helpers::spawn_app;

#[tokio::test]
async fn get_index_should_return_index_html() {
    let app = spawn_app().await;

    let response = app.get_index().await;

    assert_eq!(response.status().as_u16(), 200);
    assert!(response.text().await.unwrap().contains("瞎读"));
}
