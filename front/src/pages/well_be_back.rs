use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(WellBBack)]
pub fn well_be_back() -> Html {
    

    html! {<>
        <div class="announce_div">
            <h2 class="announce_title">
                {"Under construction please give us"}<code>{" (whos us? im alone)"}</code>
            </h2>
            <h2 class="announce_title">
                {"some time to finish!"}<code>{" (maybe like 1-3 days idfk)"}</code>
            </h2>
        </div>
        <div class="img_div">
            <img src="src/assets/kitty.jpg" title="kitty :3" alt="one of my cats staring"/>
        </div>        
    </>}
}
