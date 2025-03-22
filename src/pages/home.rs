use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::{
	Real,
	RealShow,
	RealProvider,
	
	icon::*,
	TitleAndDescription,
};

#[component]
pub fn Home() -> impl IntoView {
    view! {
    	<TitleAndDescription title="Home " desc="Home description"/>
    	<h1 title="(amateur)" class="home_page_title">
    		<sub class="title_sub darkened_text">
    			"self-taught..."
    		</sub>
    		"Software engineer"
   		</h1>
   		// make this edittable maybe
   		<section class="home_section intro">
   			<p>"Mostly specialising in web dev and design, I do freelance and take in reasonable commissions."</p>
   			<p>"As well I provide website/server hosting."</p>
   			<p>"More information can be talked over on my socials."</p>
   		</section>

    	<section class="home_section socials_container">
    		<p>"Socials"</p>
    		// business email, I don't really want to have my personal on her?
    		// <a href="mailto:???"></a>
    		<div class="socials_links">
	   			<a href="https://www.instagram.com/4izeliz/" title="Instagram ()"><Ig/></a>
	   			<a href="https://twitter.com/JrezcI" title="Twitter"><Twt/></a>
   			</div>
    	</section>

    	// maybe this will be the footer
    	// <RealProvider>
    		// <RealShow is=false>
    			// <Real/>
    		// </RealShow>
    	// </RealProvider>
    }
}
