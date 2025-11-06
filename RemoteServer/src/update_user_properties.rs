use banki_common::update_user_properties::UpdateUserPropertiesRQ;
use sqlx::query;
use tokio::sync::MutexGuard;
use anyhow::Result;

use crate::{log::LogMessage, Context};

pub async fn handle(request: UpdateUserPropertiesRQ, mut ctx: MutexGuard<'_, Context>) -> Result<bool> {
	Ok(if request.name.len() > 2 && request.name.len() < 25 {
		LogMessage::UpdateUserProperties(ctx.user.u(), &request.name).print();

		query("
			UPDATE Users
			SET name = ?1
			WHERE id = ?2
		").bind(request.name).bind(ctx.user.i()).execute(ctx.db).await?;

		let user = ctx.user;
		ctx.inform.insert(user);
		true
	}

	else {false})
}