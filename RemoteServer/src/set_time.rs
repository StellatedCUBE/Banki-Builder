use banki_common::set_time::SetTimeRQ;
use sqlx::{query, query_scalar};
use tokio::{fs, sync::MutexGuard};
use anyhow::Result;

use crate::{log::LogMessage, log_err, Context};

pub async fn handle(request: SetTimeRQ, ctx: MutexGuard<'_, Context>) -> Result<()> {
	if !query_scalar("
		SELECT COUNT(*) FROM Levels
		WHERE id = ?1
	").bind(request.level).fetch_one(ctx.db).await.inspect_err(log_err).is_ok_and(|n: u32| n == 1) {
		return Ok(());
	}

	LogMessage::SetTime(request.level, ctx.user.u(), request.time.into()).print();
	
	query("
		INSERT INTO PBs ( user, level, time )
		VALUES ( ?1, ?2, ?3 )
		ON CONFLICT
		DO UPDATE SET time = excluded.time
		WHERE excluded.time < PBs.time
	").bind(ctx.user.i()).bind(request.level).bind(request.time).execute(ctx.db).await.inspect_err(log_err)?;

	if query("
		UPDATE Levels
		SET wr = ?1, wr_holder = ?2
		WHERE id = ?3 AND wr > ?1
	").bind(request.time).bind(ctx.user.i()).bind(request.level).execute(ctx.db).await.inspect_err(log_err)?.rows_affected() == 1 {
		let tmp_file = format!("wr-runs/{:06x}.tmp", request.level);
		fs::write(&tmp_file, request.run).await.inspect_err(log_err)?;
		fs::rename(&tmp_file, &tmp_file[0..14]).await.inspect_err(log_err)?;
	}

	Ok(())
}