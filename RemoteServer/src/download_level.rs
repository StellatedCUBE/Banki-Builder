use banki_common::download_level::DownloadLevelRQ;
use tokio::{fs::read, sync::MutexGuard};
use anyhow::Result;

use crate::Context;

pub async fn handle(DownloadLevelRQ(id): DownloadLevelRQ, _ctx: MutexGuard<'_, Context>) -> Result<Vec<u8>> {
	Ok(read(format!("levels/{:06x}", id)).await?)
}