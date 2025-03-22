use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use crate::user::{
	get_user,
	ReadUser,
	WriteUser,
};
use crate::components::Loading;

// this is good enough I think idk?

#[component]
pub fn Auth(
    set_user: WriteUser,
    children: ChildrenFn,
) -> impl IntoView {
    let user = expect_context::<ReadUser>();
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

#[component]
pub fn AuthPage<F, IV>(
    set_user: WriteUser,
    fallback: F,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let user = expect_context::<ReadUser>();
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
                fallback
            >
            	{children()}
            </Show>
        </Suspense>
    }
}
