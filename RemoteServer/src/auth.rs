use std::sync::{LazyLock, RwLock};

use axum::{body::Bytes, extract::{Path, RawQuery}, http::StatusCode, response::{Html, IntoResponse, Response}};
use banki_common::{auth::{AuthToken, LogInRQ, LogInRS, LogInRSSuccess}, get_steam_openid_link::{GetSteamOpenIDLinkRQ, GetSteamOpenIDLinkRS, GetSteamOpenIDLinkRSSuccess}, is_token_pending::IsTokenPendingRQ, set_vote::Vote, Request, SelfLevelItem, SessionToken, User, UserDataPriv, MOD_VERSION};
use fxhash::FxHashMap;
use sqlx::{query, query_scalar, sqlite::SqliteRow, Row};
use steam_openid::SteamOpenId;

use crate::{db, log::LogMessage, log_err, FQDN, PROTOCOL};

static SESSIONS: LazyLock<RwLock<FxHashMap<SessionToken, User>>> = LazyLock::new(RwLock::default);

fn start_of(token: AuthToken) -> i64 {
	i64::from_ne_bytes(token[0..8].try_into().unwrap())
}

pub fn get_session(session: SessionToken) -> Option<User> {
	SESSIONS.read().unwrap().get(&session).cloned()
}

fn create_token_buffer() -> [u8; 32] {
	let mut buf = [0; 32];
	getrandom::fill(&mut buf).unwrap();
	buf
}

fn create_session_token(user: User) -> SessionToken {
	let token = create_token_buffer();
	SESSIONS.write().unwrap().insert(token, user);
	token
}

async fn create_auth_token(user: i64, trusted: bool) -> Result<AuthToken, sqlx::Error> {
	let mut token = create_token_buffer();
	let start = query("
		INSERT INTO AuthTokens ( start, user, trusted )
		VALUES ( Null, ?1, ?2 )
	").bind(user).bind(trusted).execute(db()).await?.last_insert_rowid();

	let start_buf = start.to_ne_bytes();
	for i in 0..8 {
		token[i] = start_buf[i];
	}

	query("
		UPDATE AuthTokens
		SET token = ?1
		WHERE start = ?2
	").bind(&token[..]).bind(start).execute(db()).await?;

	Ok(token)
}

pub async fn handle_login(body: Bytes) -> Response {
	if cfg!(debug_assertions) {
		println!("Q /{}", LogInRQ::PATH);
	}

	let rq: Result<(LogInRQ, _), _> = bincode::decode_from_slice(&body, bincode::config::standard());
	let rs = match rq {
		Err(_) => LogInRS::BadData,
		Ok((rq, _)) => {
			if rq.version < MOD_VERSION {
				LogInRS::OutOfDate
			}

			else {
				let dbres = match rq.token {
					Some(token) => {
						let start = start_of(token);
						let Ok(row) = query("
							SELECT token FROM AuthTokens
							WHERE start = ?1 AND user = ?2
						").bind(start).bind(rq.user.i()).fetch_optional(db()).await.inspect_err(log_err) else {
							return StatusCode::INTERNAL_SERVER_ERROR.into_response()
						};

						match row {
							Some(row) => {
								let dbtk: &[u8] = row.get(0);
								if dbtk.into_iter().cloned().eq(token.into_iter()) {
									Ok(token)
								} else {
									Err(LogInRS::BadToken)
								}
							}
							None => Err(LogInRS::BadToken)
						}
					}
					None => match query_scalar("
						SELECT COUNT(*) FROM Users
						WHERE id = ?1
					").bind(rq.user.i()).fetch_one(db()).await.inspect_err(log_err) {
						Ok(0) => match query("
							INSERT INTO Users ( id )
							VALUES ( ?1 )
						").bind(rq.user.i()).execute(db()).await {
							Err(_) => Err(LogInRS::BadToken),
							Ok(_) => {
								LogMessage::CreateUser(rq.user.u(), false).print();
								
								match create_auth_token(rq.user.i(), false).await.inspect_err(log_err) {
									Ok(token) => Ok(token),
									Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
								}
							}
						}

						Ok(1) => Err(LogInRS::BadToken),
						_ => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
					}
				};

				match dbres {
					Err(rs) => rs,
					Ok(auth_token) => LogInRS::Ok(LogInRSSuccess {
						auth_token,
						session_token: create_session_token(rq.user),
						user_data: match crate::user::get_data(vec![rq.user]).await.into_iter().next() {
							Some(data) => data,
							None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
						},
						user_data_priv: UserDataPriv {
							self_level_data: query("
								SELECT id, wr_holder, wr, likes FROM Levels
								WHERE author = ?1 AND visible
							").bind(rq.user.i())
							.map(|row: SqliteRow| SelfLevelItem {
								id: row.get(0),
								wr_holder: User::from_i(row.get(1)),
								wr_time: row.get(2),
								likes: row.get(3),
							})
							.fetch_all(db()).await.inspect_err(log_err).unwrap_or_default(),
							pbs: query("
								SELECT level, time, vote FROM PBs
								WHERE user = ?1
							").bind(rq.user.i())
							.map(|row: SqliteRow| (row.get(0), row.get(1), match row.get(2) {
								1 => Vote::Like,
								_ => Vote::None
							})).fetch_all(db()).await.inspect_err(log_err).unwrap_or_default(),
						},
					})
				}
			}
		}
	};

	match bincode::encode_to_vec(rs, bincode::config::standard()) {
		Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
		Ok(data) => data.into_response()
	}
}

fn steam_openid(id: i64, lang: u8) -> SteamOpenId {
	SteamOpenId::new(
		&format!("{}://{}/", PROTOCOL, FQDN), 
		&format!("{}://{}/{}/{}/{}", PROTOCOL, FQDN, GetSteamOpenIDLinkRQ::PATH, lang, id)
	).unwrap()
}

pub async fn handle_openid(body: Bytes) -> Response {
	if cfg!(debug_assertions) {
		println!("Q /{}", GetSteamOpenIDLinkRQ::PATH);
	}

	let rq: Result<(GetSteamOpenIDLinkRQ, _), _> = bincode::decode_from_slice(&body, bincode::config::standard());
	let rs = match rq {
		Err(_) => GetSteamOpenIDLinkRS::Err,
		Ok((rq, _)) => {
			let Ok(token) = create_auth_token(0, true).await.inspect_err(log_err) else {
				return StatusCode::INTERNAL_SERVER_ERROR.into_response()
			};

			let token_id = start_of(token);
			GetSteamOpenIDLinkRS::Ok(GetSteamOpenIDLinkRSSuccess {
				link: steam_openid(token_id, rq.language).get_redirect_url().to_owned(),
				auth_token: token,
				token_id,
			})
		}
	};

	match bincode::encode_to_vec(rs, bincode::config::standard()) {
		Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
		Ok(data) => data.into_response()
	}
}

pub async fn openid_callback(Path((lang, token_id)): Path<(u8, i64)>, RawQuery(query_string): RawQuery) -> impl IntoResponse {
	let mut success = false;

	if let Ok(user) = steam_openid(token_id, lang).verify(&query_string.unwrap_or_default()).await {
		let user = User::from_u(user);

		let _ = query("
			DELETE FROM AuthTokens
			WHERE user = ?1 AND trusted = False
		").bind(user.i()).execute(db()).await.inspect_err(log_err);

		success = query("
			UPDATE AuthTokens
			SET user = ?1
			WHERE start = ?2
		").bind(user.i()).bind(token_id).execute(db()).await.inspect_err(log_err).is_ok();

		LogMessage::CreateUser(user.u(), true).print();

		let _ = query("
			INSERT INTO Users ( id )
			VALUES ( ?1 )
		").bind(user.i()).execute(db()).await.inspect_err(log_err);
	}

	Html(format!(include_str!("open_id.html"), match (success, lang) {
		(false, 0) => "",
		(false, 1) => "Unable to log you in.",
		(false, _) => "",

		(true, 0) => "",
		(true, 1) => "Logged in successfully.<br>You may return to the game.",
		(true, _) => ""
	}))
}

pub async fn handle_is_token_pending(body: Bytes) -> [u8; 1] {
	if cfg!(debug_assertions) {
		println!("Q /{}", IsTokenPendingRQ::PATH);
	}

	let rq: Result<(IsTokenPendingRQ, _), _> = bincode::decode_from_slice(&body, bincode::config::standard());
	let rs = match rq {
		Err(_) => false,
		Ok((rs, _)) => match query_scalar("
			SELECT user FROM AuthTokens
			WHERE start = ?1
		").bind(rs.token_id).fetch_optional(db()).await {
			Err(_) => false,
			Ok(user) => user == Some(0i64)
		}
	};

	[rs as u8]
}