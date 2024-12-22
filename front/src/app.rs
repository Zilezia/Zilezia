use crate::store::Store;
use crate::router::{ switch, Route };
use crate::components::header::Header;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use stylist::yew::{
    Global,
    styled_component,
};

#[styled_component]
pub fn App() -> Html {
    // all at just have for later
    let (store, _dispatch) = use_store::<Store>();
    let is_page_loading = store.page_loading.clone();

    html! {<>
        <Global css={css!(r#"
            * { 
                margin: 0;
                padding: 0;
                list-style-type: none;
            }
            
            :root {
                font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
                line-height: 1.5;
                font-weight: 400;

                color-scheme: light dark;
                color: rgba(255, 255, 255, 0.87);
                background-color: #0D001A;

                font-synthesis: none;
                text-rendering: optimizeLegibility;
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
            }

            body {
                margin: 0 auto;
                width: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            
            h1 {
                font-size: 3.2em;
                line-height: 1.1;
            }

            a {
                font-weight: 400;
                text-decoration: inherit;
                color: #eee
            }
        "#)}/>
        
        <BrowserRouter>
            <Header/>
            <Switch<Route> render={switch}/>
            if is_page_loading {/*hm*/}
        </BrowserRouter>
    </>}
}
