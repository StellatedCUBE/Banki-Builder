#![feature(async_fn_traits)]
#![feature(unboxed_closures)]

use std::{env::set_current_dir, fmt::Display, fs, process, sync::Arc};

use axum::{body::Bytes, http::{Response, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}, Router};
use banki_common::{auth::LogInRQ, get_steam_openid_link::GetSteamOpenIDLinkRQ, is_token_pending::IsTokenPendingRQ, AuthenticatedRequest, FullResponse, Request, User};
use fxhash::FxHashSet;
use log::LogMessage;
use sqlx::{pool::PoolConnection, query, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tokio::sync::{Mutex, MutexGuard};
use tower_http::catch_panic::CatchPanicLayer;

mod auth;
mod update_user_properties;
mod publish_level;
mod unpublish_level;
mod search_levels;
mod download_level;
mod set_time;
mod set_vote;
pub mod time;
pub mod user;
pub mod log;

#[cfg(debug_assertions)]
pub const FQDN: &'static str = "127.0.0.1:7171";
#[cfg(not(debug_assertions))]
pub const FQDN: &'static str = "banki-builder.shinten.moe";

#[cfg(debug_assertions)]
pub const PROTOCOL: &'static str = "http";
#[cfg(not(debug_assertions))]
pub const PROTOCOL: &'static str = "https";

static mut DB_POOL: Option<&'static SqlitePool> = None;

pub fn db() -> &'static SqlitePool {
    unsafe {
        DB_POOL.unwrap()
    }
}

pub async fn db_conn() -> PoolConnection<Sqlite> {
    db().acquire().await.unwrap()
}

async fn set_up_db(db: &SqlitePool) {
    query("
        CREATE TABLE IF NOT EXISTS Users (
            id INT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL DEFAULT ''
        )
    ").execute(db).await.unwrap();

    query("
        CREATE TABLE IF NOT EXISTS AuthTokens (
            start INTEGER PRIMARY KEY NOT NULL,
            token BLOB,
            user INT NOT NULL,
            trusted INT NOT NULL
        )
    ").execute(db).await.unwrap();

    query("
        CREATE TABLE IF NOT EXISTS Levels (
            id INT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            author INT NOT NULL,
            tags INT NOT NULL,
            publish_time INT NOT NULL,
            character_bit INT NOT NULL,
            theme_bit INT NOT NULL,
            metadata BLOB NOT NULL,
            wr INT NOT NULL,
            wr_holder INT NOT NULL,
            visible INT NOT NULL DEFAULT True,
            likes INT NOT NULL DEFAULT 0
        )
    ").execute(db).await.unwrap();

    query("
        CREATE TABLE IF NOT EXISTS PBs (
            user INT NOT NULL,
            level INT NOT NULL,
            time INT NOT NULL,
            vote INT NOT NULL DEFAULT 0,
            PRIMARY KEY (user, level)
        ) WITHOUT ROWID
    ").execute(db).await.unwrap();
}

#[tokio::main]
async fn main() {
    LogMessage::StartServer.print();
    
    if cfg!(debug_assertions) {
        set_current_dir("/tmp").unwrap();
    }

    let _ = fs::create_dir("levels");
    let _ = fs::create_dir("author-runs");
    let _ = fs::create_dir("wr-runs");

    let db_pool = SqlitePoolOptions::new().test_before_acquire(false).connect("sqlite:./db.sqlite").await.unwrap();
    set_up_db(&db_pool).await;
    unsafe {
        DB_POOL = Some(Box::leak(Box::new(db_pool)));
    }

    let mut app = Some(
        Router::new()
        .route("/", get(|| async { Redirect::permanent("https://shinten.moe/banki-builder/") }))
        .route(&format!("/{}", LogInRQ::PATH), post(auth::handle_login))
        .route(&format!("/{}", GetSteamOpenIDLinkRQ::PATH), post(auth::handle_openid))
        .route(&format!("/{}/{{lang}}/{{token_id}}", GetSteamOpenIDLinkRQ::PATH), get(auth::openid_callback))
        .route(&format!("/{}", IsTokenPendingRQ::PATH), post(auth::handle_is_token_pending))
        .layer(CatchPanicLayer::custom(|_| -> Response<String> {process::exit(1)}))
    );

    route(&mut app, update_user_properties::handle);
    route(&mut app, publish_level::handle);
    route(&mut app, unpublish_level::handle);
    route(&mut app, search_levels::handle);
    route(&mut app, download_level::handle);
    route(&mut app, set_time::handle);
    route(&mut app, set_vote::handle);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7171").await.unwrap();
    axum::serve(listener, app.unwrap()).await.unwrap();
}

pub struct Context {
    pub user: User,
    pub inform: FxHashSet<User>,
    pub db: &'static SqlitePool,
}

fn route<T, U: (AsyncFn(T, MutexGuard<'_, Context>) -> anyhow::Result<T::Response>) + Send + Sync>(app: &mut Option<Router>, handler: U)
    where T: Request + 'static, for <'a, 'b> <U as AsyncFnMut<(T, MutexGuard<'b, Context>)>>::CallRefFuture<'a>: Send, U: 'static + Clone {
    *app = Some(app.take().unwrap().route(&format!("/{}", T::PATH), post(move |body: Bytes| async move {
        if cfg!(debug_assertions) {
            println!("Q /{}", T::PATH);
        }

        let rq: Result<(AuthenticatedRequest<T>, _), _> = bincode::decode_from_slice(&body, bincode::config::standard());

        let rs = match rq {
            Err(_) => FullResponse::BadRequest,
            Ok((arq, _)) => match auth::get_session(arq.token) {
                None => FullResponse::BadAuth,
                Some(user) => {
                    let ctx = Arc::new(Mutex::new(Context {
                        user,
                        inform: FxHashSet::default(),
                        db: unsafe { DB_POOL.unwrap() },
                    }));
                    let ctx2 = ctx.clone();

                    let Ok(rsr) = tokio::spawn(async move {
                        handler(arq.request, ctx2.try_lock().unwrap()).await
                    }).await else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    };

                    let rs = match rsr {
                        Ok(rs) => rs,
                        Err(err) => {
                            log_err(&err);
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        }
                    };

                    let Ok(ctx) = ctx.try_lock() else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    };

                    FullResponse::Ok(rs, user::get_data(ctx.inform.iter().cloned()).await)
                }
            }
        };

        match bincode::encode_to_vec(rs, bincode::config::standard()) {
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(data) => data.into_response()
        }
    })));
}

fn log_err(err: &impl Display) {
    eprintln!("ERR: {}", err);
}