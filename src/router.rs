use leptos::prelude::*;
use leptos_router::{ components::*, * };

use crate::user::*;
use crate::pages::{
	*,
	test::*,
	login::*,
	template::*,
};
use crate::components::{
	*,
	auth::*,
	error::*,
};

#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub user: Option<User>
}

#[component(transparent)]
pub fn AppRouter() -> impl IntoView {
	let state = RwSignal::new(GlobalState::default());
	let (user, set_user) = create_slice(
		state,
		|state| state.user.clone(),
		|state, new_user| state.user = new_user,
	);

	provide_context(user);
	provide_context(set_user);

	let logout = ServerAction::<Logout>::new();
	let login = ServerAction::<Login>::new();

	let _ = Effect::watch(
		move || logout.value().get(),
		move |res, _, _| {
			if let Some(res) = res{
				match res {
					Ok(_) => set_user.set(None),
					Err(_) => set_user.set(None),
				}
			};
		},
		false,
	);

	let _ = Effect::watch(
		move || login.value().get(),
		move |res, _, _| {
			if let Some(res) = res {
				match res {
					Ok(res) => set_user.set(Some(res.user.clone())),
					Err(_) => set_user.set(None),
				}
			};
		},
		false,
	);

	view! {
		<Router>
			<BotAuth/>
			<Routes fallback=|| {
				let mut outside_errors = Errors::default();
				outside_errors.insert_with_default_key(AppError::NotFound);
				view! { <ErrorTemplate outside_errors/>}
			}> 
				<Route path=path!("auth") view=PageAuth/>
				
				<MainRoutes/>
				<AuthRoute/>
				<PrivateRoutes/>
			</Routes>
		</Router>
	}
}

// TODO: rewrite the parent route views with template
#[component(transparent)]
fn MainRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/") view=||view!{<PageOutlet cookie_shown=true/>}>
            <Route path=path!("") view=Home/>
            <Route path=path!("test") view=Test/>
            <Route path=path!("projects") view=Projects/>

            <Route path=path!("privacy-policy") view=PvP/>
            <Route path=path!("pp") view=PvP/>
            <Route path=path!("pvp") view=PvP/>

            <Route path=path!("terms-of-service") view=ToS/>
            <Route path=path!("tos") view=ToS/>
        </ParentRoute>
    }.into_inner()
}

#[component(transparent)]
fn AuthRoute() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/auth") view=||view!{<PageOutlet cookie_shown=false/>}>
            <Route path=path!("login") view=LoginView/>
        </ParentRoute>
    }.into_inner()
}

#[component(transparent)]
fn PrivateRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
    	<ParentRoute path=path!("") view=||view!{<AuthOutlet cookie_shown=false/>}>
        	<Route path=path!("/panel") view=Panel/>
        </ParentRoute>
    }.into_inner()
}
