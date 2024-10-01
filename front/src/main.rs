// TODO:
//   switch to 3D site the "hub" being psx lains menu ¯\_(ツ)_/¯ based icl
//   this gonna be with wgl, i tried using wgpu but thats too complicated

// mod components;
// mod pages;

use yew::prelude::*;
// use yew_router::prelude::*;

// use pages::{
//     statuses::Statuses,
//     readme::Readme,
//     raw_view::RawView,
//     notfound::NotFound,
// };

// use components::header::Header;

// #[derive(Debug, Clone, Routable, PartialEq)]
// pub enum Route {
//     #[at("/")]
//     Statuses,
//     #[at("/README.md")] // change readme to ./ ?
//     Readme,
//     #[at("/raw_view")] // gonna be gone
//     RawView,
//     // #[at("/wgpu_test")] // whole site just gonna be it, leaving it as `showcase`?
//     // Wgpu,
//     #[not_found]
//     #[at("/404")]
//     NotFound
//     // #[at("/")]
//     // WellBBack, // chnage wbb and replace notfound 
// }

// fn switch(routes: Route) -> Html {
//     match routes {
//         Route::Statuses => html! { <Statuses /> },
//         Route::Readme => html! { <Readme /> },
//         Route::RawView => html! { <RawView /> },
//         // Route::Wgpu => html! { <Wgpu /> },
//         Route::NotFound => html! { <NotFound /> },
//         // Route::WellBBack => html! { <WellBBack /> },
//     }
// }

#[function_component(App)]
fn app() -> Html {
    html! {
    <div id="root"
        style="display:flex; justify-content: center;"
    >
        <img 
            src="src/assets/freaksaker.webp"
            style="
                height: 100vh;
            "
            alt="freaksaker"
            title="currently cooking smth rn trust"
        />
    </div>
    }
    // html! {<div id="root">
    //     <Header />
    //     <main class="main">
    //         <BrowserRouter>
    //             <Switch<Route> render={switch} />
    //         </BrowserRouter>
    //     </main>
    // </div>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}