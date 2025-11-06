use std::io::Write;

use tokio::io::{AsyncRead, AsyncReadExt};

#[cfg(not(feature = "verify"))]
pub mod data;
#[cfg(not(feature = "verify"))]
pub mod tag_data;

#[derive(Clone, Copy, PartialEq)]
pub enum Locale {
	JP, EN, ZH
}

impl Locale {
	pub fn get() -> Self {
		Self::EN
	}
}

#[derive(Clone)]
pub enum MaybeLocalized {
	Unlocalized(String),
	Localized(&'static str, &'static str, &'static str),
	LocalizedDyn(String, String, String)
}

impl MaybeLocalized {
	pub async fn read(from: &mut (dyn AsyncRead + Unpin + Send)) -> anyhow::Result<Self> {
		let len1 = from.read_u16_le().await? as usize;
		if len1 & 32768 > 0 {
			let mut buf = vec![255u8; len1 & 32767];
			from.read(&mut buf).await?;
			let str1 = String::from_utf8(buf)?;
			let len2 = from.read_u16_le().await? as usize;
			let mut buf = vec![255u8; len2];
			from.read(&mut buf).await?;
			let str2 = String::from_utf8(buf)?;
			let len3 = from.read_u16_le().await? as usize;
			let mut buf = vec![255u8; len3];
			from.read(&mut buf).await?;
			let str3 = String::from_utf8(buf)?;
			Ok(Self::LocalizedDyn(str1, str2, str3))
		} else {
			let mut buf = vec![255u8; len1];
			from.read(&mut buf).await?;
			Ok(Self::Unlocalized(String::from_utf8(buf)?))
		}
	}

	pub fn write(&self, to: &mut dyn Write) -> anyhow::Result<()> {
		match self {
			Self::Unlocalized(name) => {
				let name = name.as_bytes();
				to.write(&(name.len() as u16).to_le_bytes())?;
				to.write(name)?;
			}
			Self::LocalizedDyn(n1, n2, n3) => {
				let n1 = n1.as_bytes();
				let n2 = n2.as_bytes();
				let n3 = n3.as_bytes();
				to.write(&(n1.len() as u16 | 32768).to_le_bytes())?;
				to.write(n1)?;
				to.write(&(n2.len() as u16).to_le_bytes())?;
				to.write(n2)?;
				to.write(&(n3.len() as u16).to_le_bytes())?;
				to.write(n3)?;
			}
			_ => panic!("Const strings cannot be saved")
		}

		Ok(())
	}

	pub fn for_locale(&self, locale: Locale) -> &str {
		match self {
			Self::Unlocalized(s) => s,
			Self::Localized(en, jp, zh) => match locale {
				Locale::EN => en,
				Locale::JP => jp,
				Locale::ZH => zh
			}
			Self::LocalizedDyn(en, jp, zh) => match locale {
				Locale::EN => en,
				Locale::JP => jp,
				Locale::ZH => zh
			}
		}
	}

	pub fn for_current_locale(&self) -> &str {
		self.for_locale(Locale::get())
	}

	pub const fn for_locale_static(&self, locale: Locale) -> &'static str {
		match self {
			Self::Localized(en, jp, zh) => match locale {
				Locale::EN => en,
				Locale::JP => jp,
				Locale::ZH => zh
			}
			_ => panic!("Static access of non-static MaybeLocalized")
		}
	}

	pub fn for_current_locale_static(&self) -> &'static str {
		self.for_locale_static(Locale::get())
	}
}