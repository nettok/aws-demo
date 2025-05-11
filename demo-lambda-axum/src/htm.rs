use crate::error::ServerError;
use askama::Template;
use axum::response::Html;
use tracing::{self, instrument};

type RenderResult = Result<Html<String>, ServerError>;

#[instrument]
pub async fn get_index() -> RenderResult {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct Htm;

    let template = Htm;
    render(template)
}

pub mod journal {
    use crate::db;
    use crate::extract::ValidatedForm;
    use crate::htm::{RenderResult, render};
    use askama::Template;
    use axum::extract::Path;
    use axum::response::IntoResponse;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tracing::instrument;
    use validator::Validate;

    #[derive(Deserialize, Validate)]
    pub struct Entry {
        id: Option<String>,

        #[validate(length(min = 1, message = "Can not be empty"))]
        value: String,
    }

    #[instrument]
    pub async fn get_journal_entries() -> RenderResult {
        #[derive(Template)]
        #[template(path = "journal/journal_entries.html")]
        struct Htm {
            entries: HashMap<String, String>,
        }

        let template = Htm {
            entries: db::read_entries(),
        };
        render(template)
    }

    #[instrument(skip(entry))]
    pub async fn update_journal_entry(
        ValidatedForm(entry): ValidatedForm<Entry>,
    ) -> impl IntoResponse {
        let id = entry
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let value = &entry.value;

        db::update_entry(&id, &value);
        [("HX-Trigger", "load-journal-entries")]
    }

    #[instrument(skip(id))]
    pub async fn delete_journal_entry(Path(id): Path<String>) {
        db::delete_entry(&id);
    }
}

fn render<T>(template: T) -> RenderResult
where
    T: Template,
{
    template
        .render()
        .map(|content| Html(content))
        .map_err(|error| ServerError::from(error))
}
