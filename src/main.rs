mod data;

use data::user::{get_init_user_state, User, STORE_IS_OFFLINE};
use dotenvy_macro::dotenv;
use gloo_storage::{LocalStorage, Storage};
use yew::{platform::spawn_local, prelude::*};

static BACKEND_URL: &'static str = dotenv!("BACKEND_URL");

#[function_component(App)]
fn app() -> Html {
    let loaded = use_state(|| false);
    let user = use_state(|| None as Option<User>);

    {
        let user = user.clone();
        let loaded = loaded.clone();

        use_effect_with([loaded.clone()], move |_| {
            spawn_local(async move {
                user.set(get_init_user_state().await);

                loaded.set(true);
            });
        });
    }

    html! {
        <>
            if !*loaded {
                <h1>{ "Loading..." }</h1>
            } else {
                <h1>{ "Loaded!" }</h1>

                if let Some(user) = &*user {
                    if user.id == "" {
                        <h2>{ "<offline>" }</h2>

                        <button onclick={Callback::from(move |_| {
                            let _ = LocalStorage::set(STORE_IS_OFFLINE, false);
                            loaded.set(false);
                        })}>
                            { "Click here to sync ya shit to the 'Cloud'" }
                        </button>
                    } else {
                        <h2> { "user " } { &user.id }</h2>
                    }
                } else {
                    <p>{ "You aren't registered! You can, however, play anyway." }</p>
                    <p>{ "Registering is only for syncing your game across devices. No password, it's a fuckin' clicker game." }</p>
                    <button onclick={Callback::from(move |_| {
                        let _ = LocalStorage::set(STORE_IS_OFFLINE, true);
                        loaded.set(false);
                    })}>
                        { "Click here to play anyway" }
                    </button>
                }
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
