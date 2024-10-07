use gloo_net::http::RequestBuilder;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::BACKEND_URL;

pub static STORE_IS_OFFLINE: &'static str = "floret.is_offline";

static USER_PATH: &'static str = "/user";
static GET_USER: &'static str = "/";
static NEW_USER: &'static str = "/new";

static PARAM_ID: &'static str = "id";

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Default)]

pub struct UserMetadata {
    pub avatar_url: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub metadata: UserMetadata,
}

#[derive(Serialize)]
pub struct NewUser {
    pub id: String,
    pub metadata: UserMetadata,
}

pub async fn get_init_user_state() -> Option<User> {
    if LocalStorage::get::<bool>(STORE_IS_OFFLINE).unwrap_or(false) {
        return Some(
            LocalStorage::get::<User>("floret.user_local")
                .ok()
                .unwrap_or(User {
                    id: "".to_string(),
                    metadata: Default::default(),
                }),
        );
    }

    let id = LocalStorage::get::<String>("floret.user_id");

    match id {
        Ok(id) => {
            match RequestBuilder::new(&format!("{}{}{}", BACKEND_URL, USER_PATH, GET_USER))
                .query([(PARAM_ID, id)])
                .send()
                .await
                .ok()
            {
                Some(v) => v.json::<User>().await.ok(),
                None => None,
            }
        }
        Err(_) => LocalStorage::get::<User>("floret.user_local").ok(),
    }
}
