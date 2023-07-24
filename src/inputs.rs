use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn ControlledInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! {
        cx,
        <input type="text"
            on:input= move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev))
            }
            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
            />
            <p>"Name is: " {name} </p>
    }
}

#[component]
pub fn UncontrolledInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    //Handler called when submittin
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();

        set_name(value);
    };

    view! {
        cx,
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element />
            <input type="submit" value="Submit"/>
            <p>"Name is: " {name} </p>
        </form>
    }
}
