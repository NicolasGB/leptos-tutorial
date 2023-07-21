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
    }
}

///This shows the progress to a goal
#[component]
fn ProgressBar(
    cx: Scope,
    /// The maximum value of the progress bal
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {cx,
        <progress
            max=max
            value = progress
        />
    }
}
