// icl idk what point of this just my github
use stylist::css;
use yew::prelude::*;
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <p 
                title="Still under construction." 
                class={css!(r#"
                    color: orange;
                    position: absolute;
                    top: -10px;
                    left: 0;
                    margin: 0;
                    font-size: 3em;
                "#)}
            >
                {"🛠"}
            </p>
            <nav class="navi">
                <a class="navlink" href="/">{"status"}</a>
                <a class="navlink" href="/README.md">{"README"}</a>
                <a class="navlink" href="/raw_view" title="(frontend only)">{"Raw code"}</a>
            </nav>
        </header>
    }
}