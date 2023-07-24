mod components;
mod inputs;
mod iterations;

use crate::{components::ProgressBar, inputs::UncontrolledInput, iterations::DynamicList};
use crate::{inputs::ControlledInput, iterations::StaticList};
use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! {
        cx,

        <ControlledInput />

        <UncontrolledInput />

        <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        class:red=move || count() % 2 == 1
        >
        "Click me "
        {count}
        </button>

        // NOTE: self-closing tags like <br> need an explicit /
        <br/>

        <ProgressBar progress=count />


        <p>"Double count: " {double_count}</p>
        <ProgressBar progress= Signal::derive(cx, double_count ) />


        <br/>

        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>

        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList initial_length=10/>
    }
}
