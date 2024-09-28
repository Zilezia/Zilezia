use yew::{Component, Context, Html};
use yew::prelude::*;

#[function_component(RawView)]
pub fn raw_view() -> Html {

    // gotta loop this some way or smth icba hard coding it, just keep it atm, only showing pages

    html! {<>
        <h2 class="rv_title">{"Raw Code (frontend)"}</h2>
        <p class="rv_msg" title="I just found about this while making this page"
        >
            {"View the raw code of the"}<span title="ig">{"site"}</span>{"."}
        </p>
        <code class="front_dir">
            <p>{"src"}</p>
            <ul class="src_dir">
                <li>{"assets"}
                    <ol class="in_dir">
                        <li><a class="file_link" href="/src/assets/favicon.png">{"favicon.png"}</a></li>
                        <li><a class="file_link" href="/src/assets/kitty.jpg">{"kitty.jpg"}</a></li>
                    </ol>
                </li>
                
                <li>{"components"}
                    <ol class="in_dir">
                        <li><a class="file_link" href="/src/components/header.rs">{"header.rs"}</a></li>
                        <li><a class="file_link" href="/src/components/mod.rs">{"mod.rs"}</a></li>
                    </ol>
                </li>
                
                <li>{"pages"}
                    <ol class="in_dir">
                        <li><a class="file_link" href="/src/pages/mod.rs">{"mod.rs"}</a></li>
                        <li><a class="file_link" href="/src/pages/notfound.rs">{"notfound.rs"}</a></li>
                        <li><a class="file_link" href="/src/pages/raw_view.rs" title="inception">{"raw_view.rs"}</a></li>
                        <li><a class="file_link" href="/src/pages/readme.rs">{"readme.rs"}</a></li>
                        <li><a class="file_link" href="/src/pages/statuses.rs">{"statuses.rs"}</a></li>
                        <li><a class="file_link" href="/src/pages/well_be_back.rs">{"well_be_back.rs"}</a></li>
                    </ol>
                </li>
            </ul>
            
              
        
        </code>
        <a href="https://github.com/Zilezia/Zilezia">{"(or just view it on the repo...)"}</a>
    </>}
}
