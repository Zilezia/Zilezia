use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

use common::models::Activity;

#[function_component(Home)]
pub fn home() -> Html {
    let activs = use_state(|| Vec::new());
    {
        let activs = activs.clone();
        use_effect_with_deps(move |_| {
            let activs = activs.clone();
            spawn_local(async move {
                let client = Client::new();
                let url = "http://localhost:80/api";
                let response = client.get(url)
                    .send()
                    .await
                    .expect("Failed to fetch");
                let activ_list: Vec<Activity> = response.json().await.expect("Failed to parse JSON");
                activs.set(activ_list);
            });
        || ()
        }, ());
    }

    html! {<>
        <h2>{ "Activity" }</h2>
        <div class="statuses_container">
            { for activs.iter().map(|activ| { 
                if !activ.name.is_empty() && !activ.status.is_empty() {

                    let status_color = match activ.status.as_str() {
                        "Finished" | "Right now" => "#1dc200", // green
                        "Working" => "#ffcf00", // yellow
                        "Updating" | "Fixing" => "#1b43e8", // blue
                        "Not begun" | "Nothing" | "Dropped?" => "#260024", // dark purple
                        _ => "#b3b3b3", // grey as default
                    };
                    
                    html! {
                        <div class="status_card" id={activ.id.clone()}>
                            <a href={activ.url.clone()}>
                                <div class="top_half">
                                    <p class="item_name">{ &activ.name }</p>
                                </div>
                                <div class="bottom_half">
                                    <p class="item_status" style={format!("color: {}", status_color)}>
                                        { &activ.status }
                                    </p>
                                </div>
                            </a>
                        </div>
                    }
                } else { html! {}} // dont show
           }) }
        </div>
    </>}
}