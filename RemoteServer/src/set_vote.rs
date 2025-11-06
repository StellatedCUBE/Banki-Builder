use banki_common::set_vote::{SetVoteRQ, Vote};
use sqlx::{query, query_scalar};
use tokio::sync::MutexGuard;
use anyhow::Result;

use crate::{log::LogMessage, log_err, Context};

pub async fn handle(request: SetVoteRQ, ctx: MutexGuard<'_, Context>) -> Result<u32> {
	LogMessage::SetVote(request.level, ctx.user.u(), request.vote).print();

	Ok(if query("
		UPDATE PBs
		SET vote = ?1
		WHERE user = ?2 AND level = ?3 AND vote != ?1
	").bind(request.vote as u8).bind(ctx.user.i()).bind(request.level).execute(ctx.db).await.inspect_err(log_err)?.rows_affected() == 1 {
		query_scalar("
			UPDATE Levels
			SET likes = likes + ?1
			WHERE id = ?2 AND author != ?3
			RETURNING likes
		").bind(match request.vote {
			Vote::Like => 1,
			Vote::None => -1,
		}).bind(request.level).bind(ctx.user.i()).fetch_one(ctx.db).await.inspect_err(log_err)?
	} else {
		query_scalar("
			SELECT likes FROM Levels
			WHERE id = ?1
		").bind(request.level).fetch_optional(ctx.db).await.inspect_err(log_err)?.unwrap_or_default()
	})
}