use askama::Template;
use axum::response::Html;
use tracing::{self, instrument};

#[instrument]
pub async fn get_index() -> Html<String> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct IndexTemplate;

    let template = IndexTemplate;
    Html(template.render().unwrap()) // TODO: error handling instead of unwrap
}
