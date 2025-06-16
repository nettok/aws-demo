use crate::db::PostgresPooledConnection;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}

pub async fn get_user_by_id(db_conn: PostgresPooledConnection, id: &Uuid) -> Option<User> {
    let row = db_conn
        .query_opt("select id, name, password from users where id=$1", &[&id])
        .await
        .unwrap();

    row.map(row_to_user)
}

pub async fn get_user_by_name(db_conn: PostgresPooledConnection, name: &String) -> Option<User> {
    let row = db_conn
        .query_opt(
            "select id, name, password from users where name=$1",
            &[&name],
        )
        .await
        .unwrap();

    row.map(row_to_user)
}

fn row_to_user(row: Row) -> User {
    User {
        id: row.get("id"),
        name: row.get("name"),
        password: row.get("password"),
    }
}
