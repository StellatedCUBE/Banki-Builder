use std::{fmt::Display, sync::{atomic::{AtomicBool, Ordering}, Mutex}, time::Duration};

use banki_common::{auth::{LogInRQ, LogInRS}, AuthenticatedRequest, FullResponse, Request, SessionToken, User, MOD_VERSION};
use lazy_static::lazy_static;
use reqwest::{redirect::Policy, Client};

use super::config;

pub mod user;
pub mod openid;
pub mod level;

//#[cfg(debug_assertions)]
//const SERVER: &'static str = "http://localhost:7171";
//#[cfg(not(debug_assertions))]
const SERVER: &'static str = "https://banki-builder.shinten.moe";

const FAKE_PING: Duration = Duration::from_millis(0);

#[derive(Debug)]
pub enum Error {
	MalformedRequest,
	MalformedResponse,
	AuthFailure,
	NetworkError,
	ServerError,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AuthStatus {
	NotTried,
	Trying,
	Authed(SessionToken),
	NeedSteamAuth,
	Failed,
}

lazy_static! {
	static ref CLIENT: Client = Client::builder().user_agent("BankiBuilder").redirect(Policy::none()).build().unwrap();
}

pub static AUTH_STATUS: Mutex<AuthStatus> = Mutex::new(AuthStatus::NotTried);
pub static OUT_OF_DATE: AtomicBool = AtomicBool::new(false);
pub static NEED_UPDATE_NAME: AtomicBool = AtomicBool::new(false);

async fn get_session_token_inner() -> Result<SessionToken, Error> {
	let auth_token = config::get().auth_token;

	loop {
		let a_s;
		{
			let mut asl = AUTH_STATUS.lock().unwrap();
			a_s = *asl;
			if a_s == AuthStatus::NotTried {
				*asl = AuthStatus::Trying;
			}
		}

		return match a_s {
			AuthStatus::Authed(token) => Ok(token),
			AuthStatus::NotTried => {
				let self_id = user::SELF.load(Ordering::Relaxed);

				if self_id == 0 {
					return Err(Error::MalformedRequest);
				}

				let data = CLIENT.post(format!("{}/{}", SERVER, LogInRQ::PATH))
					.body(bincode::encode_to_vec(LogInRQ {
						user: User::from_u(self_id),
						token: auth_token,
						version: MOD_VERSION,
					}, bincode::config::standard()).unwrap())
					.send().await
					.map_err(|_| Error::NetworkError)?
					.error_for_status()
					.map_err(|_| Error::ServerError)?
					.bytes().await
					.map_err(|_| Error::NetworkError)?;

				let frs: LogInRS = bincode::decode_from_slice(&data, bincode::config::standard())
					.map_err(|_| Error::MalformedResponse)?
					.0;

				match frs {
					LogInRS::OutOfDate => {
						OUT_OF_DATE.store(true, Ordering::Relaxed);
						Err(Error::AuthFailure)
					}

					LogInRS::BadData => Err(Error::MalformedRequest),

					LogInRS::BadToken => {
						*AUTH_STATUS.lock().unwrap() = AuthStatus::NeedSteamAuth;
						Err(Error::AuthFailure)
					}

					LogInRS::Ok(rs) => {
						*AUTH_STATUS.lock().unwrap() = AuthStatus::Authed(rs.session_token);

						if rs.user_data.name == "" {
							NEED_UPDATE_NAME.store(true, Ordering::Relaxed);
						}

						if Some(rs.auth_token) != auth_token {
							config::get_mut().auth_token = Some(rs.auth_token);
							config::save();
						}

						user::learn(rs.user_data);
						level::read_initial(&rs.user_data_priv);

						Ok(rs.session_token)
					}
				}
			}
			AuthStatus::Trying => {
				tokio::time::sleep(Duration::from_secs(1)).await;
				continue
			}
			_ => Err(Error::AuthFailure)
		}
	}
}

async fn get_session_token() -> Result<SessionToken, Error> {
	get_session_token_inner().await.inspect_err(|_err| {
		//println!("{}", _err);
		let mut asl = AUTH_STATUS.lock().unwrap();
		if *asl != AuthStatus::NeedSteamAuth {
			*asl = AuthStatus::Failed;
		}
	})
}

pub async fn query<T: Request>(request: &T) -> Result<T::Response, Error> {
	loop {
		if cfg!(debug_assertions) && !FAKE_PING.is_zero() {
			tokio::time::sleep(FAKE_PING).await;
		}

		let data = CLIENT.post(format!("{}/{}", SERVER, T::PATH))
			.body(bincode::encode_to_vec(AuthenticatedRequest {
				request: request.clone(),
				token: get_session_token().await?
			}, bincode::config::standard()).unwrap())
			.send().await
			.map_err(|_| Error::NetworkError)?
			.error_for_status()
			.map_err(|_| Error::ServerError)?
			.bytes().await
			.map_err(|_| Error::NetworkError)?;

		let frs: FullResponse<T::Response> = bincode::decode_from_slice(&data, bincode::config::standard())
			.map_err(|_| Error::MalformedResponse)?
			.0;

		return match frs {
			FullResponse::BadRequest => Err(Error::MalformedRequest),
			FullResponse::BadAuth => {
				*AUTH_STATUS.lock().unwrap() = AuthStatus::NotTried;
				continue
			}
			FullResponse::Ok(rs, users) => {
				for user in users {
					user::learn(user);
				}

				Ok(rs)
			}
		};
	}
}

pub async fn setup() {
	let _ = get_session_token().await;
}

pub fn logged_in() -> bool {
	match *AUTH_STATUS.lock().unwrap() {
		AuthStatus::Authed(_) => true,
		_ => false,
	}
}