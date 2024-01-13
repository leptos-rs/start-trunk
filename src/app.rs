use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
