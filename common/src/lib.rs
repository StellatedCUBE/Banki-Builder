use std::num::{NonZeroU32, NonZeroU64};

use bincode::{Decode, Encode};

pub mod auth;
pub mod update_user_properties;
pub mod get_steam_openid_link;
pub mod is_token_pending;
pub mod publish_level;
pub mod unpublish_level;
pub mod search_levels;
pub mod download_level;
pub mod set_time;
pub mod set_vote;
pub mod id;

pub const MOD_VERSION: u16 = 2;

pub trait Request: Encode + Decode<()> + Send + Clone {
    type Response: Response;
    const PATH: &'static str;
}

pub trait Response: Encode + Decode<()> + Send {}

pub type SessionToken = [u8; 32];

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct User(NonZeroU64);

impl User {
    pub const fn u(self) -> u64 {
        self.0.get()
    }

    pub const fn i(self) -> i64 {
        self.0.get() as i64
    }

    pub const fn from_u(id: u64) -> Self {
        Self(NonZeroU64::new(id).unwrap())
    }

    pub const fn from_i(id: i64) -> Self {
        Self::from_u(id as u64)
    }
}

#[derive(Encode, Decode)]
pub struct OnlineLevelMetadata {
    pub wr_time: NonZeroU32,
    pub wr_holder: User,
    pub likes: u32,
    pub metadata_blob: Box<[u8]>,
}

#[derive(Encode, Decode)]
pub struct AuthenticatedRequest<T> where T: Request {
    pub token: SessionToken,
    pub request: T,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct UserData {
    pub id: User,
    pub name: String,
}

#[derive(Encode, Decode, Copy, Clone, Debug)]
pub struct SelfLevelItem {
    pub id: u32,
    pub wr_time: NonZeroU32,
    pub wr_holder: User,
    pub likes: u32,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct UserDataPriv {
    pub self_level_data: Vec<SelfLevelItem>,
    pub pbs: Vec<(u32, NonZeroU32, set_vote::Vote)>,
}

#[derive(Encode, Decode)]
pub enum FullResponse<T> where T: Response {
    Ok(T, Vec<UserData>),
    BadRequest,
    BadAuth,
}

impl Response for () {}
impl Response for bool {}
impl Response for u32 {}
impl Response for Vec<u8> {}
