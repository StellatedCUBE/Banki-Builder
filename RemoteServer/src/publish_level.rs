use banki_common::{id::id_to_code, publish_level::{PublishLevelRQ, VerificationResponse}};
use rand::random;
use serde::Serialize;
use sqlx::query;
use tokio::{fs, io::AsyncWriteExt, process::Command, sync::MutexGuard};
use anyhow::Result;

use crate::{log::LogMessage, log_err, time::now, user, Context};

const BAD: u32 = u32::MAX - 1;

const DISCORD_WEBHOOK: &'static str = "example://discord.com/api/webhooks/INSERT_HERE";

fn escape_discord(x: &str) -> String {
	let mut out = String::new();
	let mut last = '\0';

	for c in x.chars() {
		if c == '_' || c == '*' || c == '~' || c == ':' || c == '\\' {
			out.push('\\');
			out.push(c);
		} else if c == '(' && last == ']' {
			out.push_str("\u{200b}(");
		} else if c.is_whitespace() {
			out.push(' ');
		} else if !c.is_control() {
			out.push(c);
		}

		if c == '#' || c == '@' {
			out.push('\u{200b}');
		}

		last = c;
	}

	out
}

#[derive(Serialize)]
struct DiscordForm {
	content: String
}

pub async fn handle(request: PublishLevelRQ, ctx: MutexGuard<'_, Context>) -> Result<u32> {
	let mut id: u32;
	let mut level_path;
	loop {
		id = random::<u32>() & 0xffffffu32;
		level_path = format!("levels/{:06x}", id);
		let level_fh = fs::File::create_new(&level_path).await;
		if let Ok(mut level_fh) = level_fh {
			level_fh.write_all(&request.level).await?;
			break;
		}
	}

	let Ok(buf) = Command::new("./verify-level").arg(&level_path).arg(id.to_string()).arg(ctx.user.u().to_string()).output().await else {
		let _ = fs::remove_file(level_path).await;
		return Ok(BAD)
	};

	let Ok((response, _)) = bincode::decode_from_slice::<VerificationResponse, _>(&buf.stdout, bincode::config::standard()) else {
		let _ = fs::remove_file(level_path).await;
		return Ok(BAD)
	};

	fs::write(format!("author-runs/{:06x}", id), request.verification_run).await?;
	fs::symlink(format!("../author-runs/{:06x}", id), format!("wr-runs/{:06x}", id)).await?;

	if let Some(user) = user::get_data([ctx.user]).await.first() {
		let _ = x_http_curl::post(DISCORD_WEBHOOK, &serde_urlencoded::to_string(DiscordForm {
			content: format!(
				"<@&1360562630309314660>\nNew level: <:{}> **{}** ({})\nBy: [{}](<https://steamdb.info/calculator/{}/>)",
				match response.character_bit {
					1 => "HeadBanki:1360591984267628634",
					2 => "HeadCirno:1360591988747010140",
					4 => "HeadRumia:1360591987283329036",
					8 => "HeadSeija:1360591986146541639",
					_ => ""
				},
				escape_discord(&response.name),
				id_to_code(id),
				escape_discord(&user.name),
				ctx.user.u(),
			)
		}).unwrap());
	}

	LogMessage::CreateLevel(id, &response.name, ctx.user.u(), response.tags, response.character_bit, response.theme_bit, request.verification_time).print();

	query("
		INSERT INTO Levels ( id, name, author, tags, publish_time, character_bit, theme_bit, metadata, wr, wr_holder )
		VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?3 )
	").bind(id)
	.bind(response.name)
	.bind(ctx.user.i())
	.bind(response.tags)
	.bind(now())
	.bind(response.character_bit)
	.bind(response.theme_bit)
	.bind(response.metadata_buf)
	.bind(request.verification_time)
	.execute(ctx.db).await
	.inspect_err(log_err)?;

	LogMessage::SetTime(id, ctx.user.u(), request.verification_time).print();

	query("
		INSERT INTO PBs ( user, level, time )
		VALUES ( ?1, ?2, ?3 )
	").bind(ctx.user.i()).bind(id).bind(request.verification_time).execute(ctx.db).await.inspect_err(log_err)?;

	Ok(id)
}
