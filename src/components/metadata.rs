use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::*;

#[component]
pub fn MetaData() -> impl IntoView {
    provide_meta_context();
    let formatter = |text| format!("{text}Zilezia");
    view! {
   		<Title formatter/>
        <Style>{include_str!("../styles.css")}</Style>
        <Link rel="icon" type_="image/svg+xml" href="/images/icon.png" />
        // do more metatags?
        
        // <Link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png?v=0" />
        // <Link rel="icon" type_="image/png" sizes="32x32" href="/static/favicon-32x32.png?v=0" />
        // <Link rel="icon" type_="image/png" sizes="16x16" href="/static/favicon-16x16.png?v=0" />
        // <Link rel="manifest" href="/static/site.webmanifest?v=0" />
        // <Link rel="mask-icon" href="/static/safari-pinned-tab.svg?v=0" attr:color="#4f46e5" />
        // <Link rel="shortcut icon" href="/static/favicon.ico?v=0" />
        // <Meta name="msapplication-TileColor" content="#4f46e5" />
        // <Meta name="msapplication-config" content="/static/browserconfig.xml?v=0" />
        <Meta name="theme-color" content="#0d001a"/>
        <Link rel="preload" href="/fonts/Michroma/Michroma-Regular.ttf" crossorigin="anonymous"/>
    }
}

#[component]
pub fn TitleAndDescription(
    #[prop(into)] title: TextProp,
    #[prop(into)] desc: TextProp,
) -> impl IntoView {
    view! {
        <Title text=title/>
        <Meta name="description" content=desc/>
    }
}
