use crate::error::{self, AppError};
use askama::Template;
use axum::extract::Request;
use axum::response::Html;
use tracing::{self, instrument};

#[instrument(skip(request))]
pub async fn get_index(request: Request) -> Result<Html<String>, AppError> {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexTemplate;

    let template = IndexTemplate;
    render(template, request)
}

fn render<T>(template: T, request: Request) -> Result<Html<String>, AppError>
where
    T: Template,
{
    template
        .render()
        .map(|content| Html(content))
        .map_err(|error| error::template_error(request, error))
}
