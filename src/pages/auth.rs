use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use leptos_router::hooks::use_location;

use crate::user::*;
use crate::api::auth::*;
use crate::components::{
	*,
	icon::*,
	error::*,
};

#[component]
pub fn AuthPage(children: ChildrenFn) -> impl IntoView {	
    let user = expect_context::<ReadUser>();
    let set_user = expect_context::<WriteUser>();
    let resource_user = Resource::new(|| (), |_| async { get_user().await });

    let is_authed = Signal::derive(move || match resource_user.get() {
		Some(res) => match res {
            Ok(user) => {
                set_user.set(Some(user));
                true
            }
            _ => {
                set_user.set(None);
                false
            }
        },
        _ => false
    });

    view! {
        <Suspense fallback=Loading>
            <Show
                when=move || { is_authed.get() }
                fallback=|| {
           			let mut outside_errors = Errors::default();
               		outside_errors.insert_with_default_key(
                   		AppError::Unauthorized("Invalid Session ID".into())
               		);
               		view! { <ErrorTemplate outside_errors/>}
               	}
            >{children()}</Show>
        </Suspense>
    }
}


#[component]
pub fn PageAuth() -> impl IntoView {
    view! {
    	<TitleAndDescription title="Auth " desc=""/>
    	
    	<AuthProvider attr:style="width:100%;height:100vh;">
       		<AuthShow is=false>
	       		<Auther/>
    	   	</AuthShow>
		</AuthProvider>
    }
}

#[component(transparent)]
pub fn BotAuth() -> impl IntoView {
   	let location = use_location();
    let current_route = move || location.pathname.get();

    let resource = Resource::new(
        move || current_route(),
        |current_route| async move {
            match check_auth_api(current_route).await {
                Ok(_) => true,
                Err(_) => false,
            }
        },
    );

    view! {
        <Suspense fallback=Loading>
            <Show when=move || { resource.get().expect("reason") }>
            	<div></div>
            </Show>
        </Suspense>
	}
}

#[island]
pub fn AuthProvider(children: Children) -> impl IntoView {
	provide_context(RwSignal::new(false));
	children()
}

#[island]
pub fn Auther() -> impl IntoView {
	let (card_toggle, set_card_show) = expect_context::<RwSignal<bool>>().split();
	let action = ServerAction::<SetAuthApi>::new();
	view! {
		<ActionForm action=action>
			<button
				type="submit"
				id="person_auth"
				class="btnless"
				on:click=move |_| {
			        // spawn_local(async move {
			        	// set_auth_api().await;
			        // });
   					set_card_show.update(|n| *n = !*n);
				}
			>
				<p class="darkened_text person_auth_text">"(press anywhere)"</p>
				<Fur/>
				<p class="darkened_text person_auth_text">"(press anywhere)"</p>
			</button>
		</ActionForm>
	}
}

#[island]
pub fn AuthShow(is: bool, children: Children) -> impl IntoView {
	let card_toggle = expect_context::<RwSignal<bool>>().read_only();
    view! {
	    <div
	    	style="transition: .5s"
	    	style:opacity=move || if card_toggle.get() == is {"0"} else {"1"}
	    	style:display=move || if card_toggle.get() == is {"contents"} else {"none"}
        >{children()}</div>
    }
}
