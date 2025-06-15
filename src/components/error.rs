use http::status::StatusCode;
use leptos::prelude::*;
use thiserror::Error;

use crate::pages::BotAuth;
use crate::components::TitleAndDescription;

#[derive(Clone, Debug, Error)]
pub enum AppError {
	// 4xx
    #[error("Not Found")]
    NotFound,
    // the pain of different localisation and wanting the errors to be the same (icba writing my own whole error system, just build off StatusCode)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
	// 5xx
    #[error("Internal Error: {0}")]
    InternalError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Invalid data provided: {0}")]
    InvalidData(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
        	// 4xx
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
			// 5xx
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidData(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    
    let errors = errors.get_untracked();

    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();

    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    view! {
    	<BotAuth/>
    	<TitleAndDescription title=format!("{} ", errors[0].status_code()) desc=""/>
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <For
            each=move || { errors.clone().into_iter().enumerate() }
            key=|(index, _error)| *index
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}
