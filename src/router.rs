use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

use crate::pages::*;
use crate::pages::test::*;
use crate::pages::login::*;
use crate::components::auth::*;

use crate::components::*;
use crate::user::*;
use crate::error::*;

#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub user: Option<User>
}

pub fn make_url<T: AsRef<str>>(base_path: T, query_string: &str) -> String {
    let root = "http://localhost:3000";

    if !query_string.is_empty() {
        format!("{}{}?{}", root, base_path.as_ref(), query_string)
    } else {
        format!("{}{}", root, base_path.as_ref())
    }
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
				
				<MainRoutes set_user logout/>
				<AuthRoute set_user login logout/>
				<PrivateRoutes set_user logout/>
			</Routes>
		</Router>
	}
}

#[component(transparent)]
fn MainRoutes(set_user: WriteUser, logout: ServerAction<Logout>) -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/") view=move || view!{
        	<Header logout set_user/>
        	<main id="main_container">
        		<Outlet/>
        	</main>
        	<Cookie/>
       	}>
            <Route path=path!("") view=Home/>
            <Route path=path!("test") view=Test/>
            <Route path=path!("projects") view=move || view!{ <Projects set_user/> }/>

            <Route path=path!("privacy-policy") view=PvP/>
            <Route path=path!("pp") view=PvP/>
            <Route path=path!("pvp") view=PvP/>

            <Route path=path!("terms-of-service") view=ToS/>
            <Route path=path!("tos") view=ToS/>
        </ParentRoute>
    }.into_inner()
}

#[component(transparent)]
fn AuthRoute(set_user: WriteUser, login: ServerAction<Login>, logout: ServerAction<Logout>) -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/auth") view=move || view!{
        	<Header logout set_user/>
        	<main id="main_container">
        		<Outlet/>
        	</main>
       	}>
            <Route
                path=path!("login")
                view=move || view! {<LoginView action=login/>}
            />
        </ParentRoute>
    }.into_inner()
}

#[component(transparent)]
fn PrivateRoutes(set_user: WriteUser, logout: ServerAction<Logout>) -> impl MatchNestedRoutes + Clone {
    view! {
    	<ParentRoute path=path!("") view=move || view! {
   			<AuthPage
				set_user
                fallback=|| {
					let mut outside_errors = Errors::default();
              		outside_errors.insert_with_default_key(
                    	AppError::Unauthorized("Invalid Session ID".into())
                  	);
         			view! { <ErrorTemplate outside_errors/>}
         		}
   			>
   				<Header logout set_user/>
	        	<main id="main_container">
	        		<Outlet/>
	        	</main>
			</AuthPage>
    	}>
        	<Route path=path!("/panel") view=Panel/>
        </ParentRoute>
    }.into_inner()
}
