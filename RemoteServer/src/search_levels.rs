use banki_common::{search_levels::*, OnlineLevelMetadata, User};
use sqlx::{query, Row};
use tokio::sync::MutexGuard;
use anyhow::Result;

use crate::{Context, log_err};

pub async fn handle(request: SearchLevelsRQ, mut ctx: MutexGuard<'_, Context>) -> Result<SearchLevelsRS> {
	let db = ctx.db;
	let q;
	let results = if request.id == u32::MAX {
		q = format!("
			SELECT metadata, wr, wr_holder, likes, author FROM Levels
			WHERE visible AND character_bit & ?1 AND theme_bit & ?2 AND (tags & ?3) = ?3 AND (tags & ?4) = 0
			ORDER BY {} DESC
			LIMIT 256
		", match request.order {
			LevelOrdering::New => "publish_time",
			LevelOrdering::Top => "likes",
		});
		query(&q).bind(request.characters).bind(request.themes).bind(request.tags).bind(request.neg_tags)
	} else {
		query("
			SELECT metadata, wr, wr_holder, likes, author FROM Levels
			WHERE id = ?1 AND visible
		").bind(request.id)
	}.map(|row| {
		ctx.inform.insert(User::from_i(row.get(4)));
		OnlineLevelMetadata {
			metadata_blob: row.get(0),
			wr_time: row.get(1),
			wr_holder: User::from_i(row.get(2)),
			likes: row.get(3),
		}
	}).fetch_all(db).await.inspect_err(log_err)?;

	for olm in &results {
		ctx.inform.insert(olm.wr_holder);
	}

	Ok(SearchLevelsRS {
		results,
		more: false,
	})
}