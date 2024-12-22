use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component]
pub fn Header() -> Html {
    html! {
        <header class={css!(r#"
            margin: .4em .5em;
            width: calc(100% - 1em);
            display: flex;
            flex-direction: row;
            justify-content: flex-end;
            gap: .4em;
        "#)}>
            <Link<Route> 
                to={Route::HomePage} 
                classes={css!(r#"
                    font-size: 1em;
                    background-color: #B300B3;
                    border-radius: 5%;
                    padding: .15em .2em;
                    :hover { background-color: #800080; }
            "#)}>
                {"Home"}
            </Link<Route>>
        </header>
    }
}
