use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// A router which renders the homepage and handles 404's
#[component]
fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        <Router>
            <Routes>
                <Route path="" view=Home /> />
                <Route path="/*" view=|| view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  } />
            </Routes>
        </Router>
    }
}

/// Default homepage
#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

            <h1>"Welcome to Leptos"</h1>

            <div class="buttons">
                <Button />
                <Button increment=5 />
            </div>

        </div>
    }
}

/// A parameterized incrementing button
#[component]
fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: " {count}
        </button>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
