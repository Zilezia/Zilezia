use leptos::prelude::*;
use leptos_meta::*;
use crate::router::AppRouter;
use crate::components::MetaData;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone() islands=true/>
                <MetaTags/>
                // <HashedStylesheet id="leptos" options/> // where is this?
            </head>
            <body id="root">
                <App/>
            </body>
        </html>
    }
}


#[component]
#[must_use]
pub fn App() -> impl IntoView {
	view! {
		<MetaData/>
		<AppRouter/>
	}
}
