use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod home;
pub mod not_found;
use crate::home::Home;
use crate::not_found::NotFound;

/// A router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        <Router>
            <Routes>
                <Route path="/" view=App />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
