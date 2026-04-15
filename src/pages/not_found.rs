use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <h1>"404: Page not found"</h1>
        <h2>"We couldn't find that page!"</h2>
    }
}
