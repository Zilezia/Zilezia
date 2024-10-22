use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    

    html! {<>
        <div class="img_div">
            <img src="src/assets/kitty.jpg" title="kitty :3" alt="one of my cats staring"/>
        </div>        
    </>}
}
