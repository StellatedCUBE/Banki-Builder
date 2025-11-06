use banki_common::{get_steam_openid_link::{GetSteamOpenIDLinkRQ, GetSteamOpenIDLinkRS, GetSteamOpenIDLinkRSSuccess}, is_token_pending::IsTokenPendingRQ, Request};

use crate::controller::loc::Locale;

use super::{user::me, Error, CLIENT, SERVER};

pub async fn get_openid_link() -> Result<GetSteamOpenIDLinkRSSuccess, Error> {
	let data = CLIENT.post(format!("{}/{}", SERVER, GetSteamOpenIDLinkRQ::PATH))
		.body(bincode::encode_to_vec(GetSteamOpenIDLinkRQ {
			user: me(),
			language: Locale::get() as u8,
		}, bincode::config::standard()).unwrap())
		.send().await
		.map_err(|_| Error::NetworkError)?
		.error_for_status()
		.map_err(|_| Error::ServerError)?
		.bytes().await
		.map_err(|_| Error::NetworkError)?;

	let frs: GetSteamOpenIDLinkRS = bincode::decode_from_slice(&data, bincode::config::standard())
		.map_err(|_| Error::MalformedResponse)?
		.0;

	match frs {
		GetSteamOpenIDLinkRS::Err => Err(Error::MalformedRequest),
		GetSteamOpenIDLinkRS::Ok(rs) => Ok(rs)
	}
}

pub async fn is_token_pending(token_id: i64) -> bool {
	let rs = CLIENT.post(format!("{}/{}", SERVER, IsTokenPendingRQ::PATH))
		.body(bincode::encode_to_vec(IsTokenPendingRQ {
			token_id
		}, bincode::config::standard()).unwrap())
		.send().await
		.and_then(|rs| rs.error_for_status());
		
	let Ok(rs) = rs else { return false };
	let Ok(data) = rs.bytes().await else {return false; };

	data.into_iter().next() == Some(1)
}