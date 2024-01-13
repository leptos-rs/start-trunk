use leptos::*;

#[component]
fn App(increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="container">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

            <h1>"Welcome to Leptos"</h1>

            <button
                on:click= move |_| {
                    set_count(count() + increment)
                }
            >
                "Click me: "
                {count}
            </button>


        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App increment=5 />
        }
    })
}
