use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

use common::models::User;

#[function_component(App)]
fn app() -> Html {
    let users = use_state(|| Vec::new());

    {
        let users = users.clone();
        use_effect_with_deps(move |_| {
            let users = users.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client.get("http://127.0.0.1:8080/api/users")
                    .send()
                    .await
                    .expect("Failed to fetch");
                let users_list: Vec<User> = response.json().await.expect("Failed to parse JSON");
                users.set(users_list);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <h1>{ "Users" }</h1>
            <ul>
                { for users.iter().map(|user| html! {
                    <li>{ format!("ID: {}, Username: {}", user.id, user.username) }</li>
                }) }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
