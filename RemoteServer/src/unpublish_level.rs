use banki_common::unpublish_level::UnpublishLevelRQ;
use sqlx::query;
use tokio::sync::MutexGuard;
use anyhow::Result;

use crate::{log::LogMessage, log_err, Context};

pub async fn handle(request: UnpublishLevelRQ, ctx: MutexGuard<'_, Context>) -> Result<()> {
	if query("
		UPDATE Levels
		SET visible = False
		WHERE id = ?1 AND author = ?2
	").bind(request.level).bind(ctx.user.i()).execute(ctx.db).await.inspect_err(log_err)?.rows_affected() == 1 {
		LogMessage::DeleteLevel(request.level).print();
		Ok(())
	} else {
		Err(anyhow::Error::msg(""))
	}
}