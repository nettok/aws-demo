use askama::Template;
use axum::response::Html;

pub async fn get_index() -> Html<String> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Index;

    let template = Index;
    Html(template.render().unwrap())
}
