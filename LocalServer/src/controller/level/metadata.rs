
use std::time::SystemTime;

use anyhow::anyhow;
use bincode::{Decode, Encode};
use tokio::io::{AsyncRead, AsyncReadExt};

#[cfg(not(feature = "verify"))]
use crate::controller::net::{level::pb, logged_in, user::is_me};

use super::{tag::Tag, Character, LevelTheme, MaybeLocalized, LEVEL_FILE_MAGIC_NUMBERS};

#[derive(Encode, Decode, Clone)]
pub struct LevelMetadata {
	pub name: String,
	pub author: u64,
	pub character: Character,
	pub tags: u32,
	pub seija_flags: u8,
	pub modified_time: SystemTime,
	pub filename: String,
	pub theme: LevelTheme,
	pub online_id: u32,
}

impl LevelMetadata {
	pub async fn load(from: &mut (dyn AsyncRead + Unpin + Send)) -> anyhow::Result<Self> {
		let mut buf = [0u8; 8];
		from.read(&mut buf).await?;

		if buf != LEVEL_FILE_MAGIC_NUMBERS {
			return Err(anyhow!("Not a valid level file"));
		}

		let _header_length = from.read_u16_le().await?;
		let _version = from.read_u16_le().await?;
		
		let author = from.read_u64_le().await?;
		let online_id = from.read_u32_le().await?;

		let _body_lens = from.read_u64_le().await?;

		let flags = from.read_u8().await?;
		let seija_flags = flags & 15;

		let character = from.read_u8().await?;
		let character = Character::parse(character)?;

		let _heads = from.read_u8().await? as u32;
		
		let theme = from.read_u8().await?;
		let theme = LevelTheme::parse(theme)?;

		let tags = from.read_u32_le().await?;

		let name = MaybeLocalized::read(from).await?.for_current_locale().to_owned();
		
		let filename = String::new();
		let modified_time = SystemTime::UNIX_EPOCH;

		Ok(Self {
			name,
			character,
			author,
			modified_time,
			filename,
			tags,
			seija_flags,
			theme,
			online_id,
		})
	}

	pub fn tagged(&self, tag: Tag) -> bool {
		self.tags & tag.bit() > 0
	}

	#[cfg(not(feature = "verify"))]
	pub fn can_vote(&self) -> bool {
		logged_in() &&
		!is_me(self.author) &&
		pb(self.online_id).is_some()
	}
}