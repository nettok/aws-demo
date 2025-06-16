use crate::error::AppError;
use askama::Template;
use axum::extract::Path;
use axum::response::Html;
use chrono::NaiveDate;
use util::tracing::{self, instrument};

type RenderResult = Result<Html<String>, AppError>;

#[instrument]
pub async fn get_index(Path(date): Path<NaiveDate>) -> RenderResult {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct Htm {
        date: NaiveDate,
    }

    let template = Htm { date };
    render(template)
}

pub mod journal {
    use crate::db::entries::Entry;
    use crate::db::{self, DatabaseConnection};
    use crate::extract::ValidatedForm;
    use crate::htm::{RenderResult, render};
    use askama::Template;
    use axum::extract::Path;
    use axum::response::IntoResponse;
    use chrono::NaiveDate;
    use serde::Deserialize;
    use util::tracing::{self, instrument};
    use uuid::Uuid;
    use validator::Validate;

    #[derive(Deserialize, Validate)]
    pub struct EntryForm {
        id: Option<String>,

        #[validate(length(min = 1, message = "Can not be empty"))]
        value: String,
    }

    #[derive(Deserialize)]
    pub struct DateAndId {
        date: NaiveDate,
        id: Uuid,
    }

    #[instrument]
    pub async fn get_journal_entries(
        DatabaseConnection(db_conn): DatabaseConnection,
        Path(date): Path<NaiveDate>,
    ) -> RenderResult {
        #[derive(Template)]
        #[template(path = "journal/journal_entries.html")]
        struct Htm {
            entries: Vec<Entry>,
        }

        // TODO: get user_id from session
        let user_id = Uuid::parse_str("ec388d97-03b6-4957-bea5-4aceae499ef4").unwrap();

        let template = Htm {
            entries: db::entries::read_entries(db_conn, &user_id, &date).await,
        };
        render(template)
    }

    #[instrument(skip(entry))]
    pub async fn update_journal_entry(
        DatabaseConnection(db_conn): DatabaseConnection,
        Path(date): Path<NaiveDate>,
        ValidatedForm(entry): ValidatedForm<EntryForm>,
    ) -> impl IntoResponse {
        // TODO: get user_id from session
        let user_id = Uuid::parse_str("ec388d97-03b6-4957-bea5-4aceae499ef4").unwrap();

        let id = entry
            .id
            .clone()
            .map(|str| Uuid::parse_str(&str).unwrap())
            .unwrap_or_else(|| Uuid::now_v7());
        let value = &entry.value;

        db::entries::update_entry(db_conn, &user_id, &date, &id, &value).await;
        [("HX-Trigger", "load-journal-entries")]
    }

    #[instrument(skip(params))]
    pub async fn delete_journal_entry(
        DatabaseConnection(db_conn): DatabaseConnection,
        Path(params): Path<DateAndId>,
    ) {
        // TODO: get user_id from session
        let user_id = Uuid::parse_str("ec388d97-03b6-4957-bea5-4aceae499ef4").unwrap();

        db::entries::delete_entry(db_conn, &user_id, &params.date, &params.id).await;
    }
}

fn render<T>(template: T) -> RenderResult
where
    T: Template,
{
    template
        .render()
        .map(|content| Html(content))
        .map_err(|error| AppError::from(error))
}
