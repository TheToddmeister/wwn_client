use http::status::InvalidStatusCode;
use http::StatusCode;
use leptos::{component, IntoView, View, view};
use serde::{Deserialize, Serialize};


#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum HttpError {
    #[error("{0} {1} {2}")]
    ServerError(u16, StatusCode, String),
    #[error("{0} {1} {2}")]
    ClientError(u16, StatusCode, String),
    #[error("{0} {1} {2} ")]
    UndefinedHttpError(u16, http::StatusCode, String),
    #[error("{0}")]
    InvalidStatusCode(#[from] InvalidStatusCode),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    HttpError(#[from] HttpError),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    GlooError(#[from] gloo_net::Error),
    #[error("Did not receive the expected data from server ")]
    EmptyDataResponseError,
}

#[component]
impl IntoView for Error {
    fn into_view(self) -> View {
        let message = "Error".to_string() + &self.to_string();
        let html_error = view! {
            <p> "Error: ". </p>
        };
        html_error.into_view()
    }
}




