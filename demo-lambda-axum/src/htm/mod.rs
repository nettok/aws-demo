use crate::error::AppError;
use askama::Template;
use axum::response::Html;

pub mod journal;
pub mod login;

pub type RenderResult = Result<Html<String>, AppError>;

pub fn render<T>(template: T) -> RenderResult
where
    T: Template,
{
    template
        .render()
        .map(|content| Html(content))
        .map_err(|error| AppError::from(error))
}
