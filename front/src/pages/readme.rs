use stylist::{Style, css};
use yew::{Component, Context, Html};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!(r"..\static\readme.css");
// const MD: &str = include_str!(r"..\..\..\REPO.md");
const MD: &str = include_str!(r"..\..\..\README.md");
// 
pub struct ReadmeMD;
impl Component for ReadmeMD {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let content = markdown::to_html(MD); // kinda alr ig?
        Html::from_html_unchecked(content.into())
    }
}

#[function_component(Readme)]
pub fn readme() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // might just rewrite readme in here as simple html, the conversion is meh
    html! {<>
        <code class={css!(r#"font-size: 2em;"#)}>{"README.md:"}</code>
        <div class={classes!("readme_cont", stylesheet)}>
            <ReadmeMD />
        </div>
    </>}
}
