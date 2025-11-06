use banki_common::{User, UserData};
use sqlx::{query, Row, sqlite::SqliteRow};

use crate::db;

pub async fn get_data(of: impl IntoIterator<Item = User>) -> Vec<UserData> {
	query(&format!("
		SELECT id, name FROM Users
		WHERE id IN ({})
	", of.into_iter().map(|u| u.i().to_string()).collect::<Vec<_>>().join(","))).map(|row: SqliteRow| UserData {
		id: User::from_i(row.get(0)),
		name: row.get(1),
	}).fetch_all(db()).await.unwrap_or(vec![])
}