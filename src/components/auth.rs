use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use crate::user::{
	get_user,
	ReadUser,
	WriteUser,
};
use crate::components::{ Fur, Loading };

#[component]
pub fn Auth(children: ChildrenFn) -> impl IntoView {
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
        <Suspense fallback=Loading> // Loading
            <Show when=move || { is_authed.get() }>
            	{children()}
            </Show>
        </Suspense>
    }
}
