use crate::db::PostgresPooledConnection;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct Entry {
    pub date: NaiveDate,
    pub id: Uuid,
    pub content: String,
}

pub async fn read_entries(
    db_conn: PostgresPooledConnection,
    user_id: &Uuid,
    date: &NaiveDate,
) -> Vec<Entry> {
    let rows = db_conn
        .query(
            "select id, content from entries where user_id=$1 and date=$2",
            &[&user_id, &date],
        )
        .await
        .unwrap();

    rows.iter()
        .map(|row| Entry {
            date: date.clone(),
            id: row.get("id"),
            content: row.get("content"),
        })
        .collect()
}

pub async fn update_entry(
    db_conn: PostgresPooledConnection,
    user_id: &Uuid,
    date: &NaiveDate,
    id: &Uuid,
    content: &String,
) {
    db_conn
        .execute(
            "insert into entries (user_id, date, id, content) values ($1, $2, $3, $4)",
            &[&user_id, &date, id, content],
        )
        .await
        .unwrap();
}

pub async fn delete_entry(
    db_conn: PostgresPooledConnection,
    user_id: &Uuid,
    date: &NaiveDate,
    id: &Uuid,
) {
    db_conn
        .execute(
            "delete from entries where user_id=$1 and date=$2 and id=$3",
            &[&user_id, &date, id],
        )
        .await
        .unwrap();
}
