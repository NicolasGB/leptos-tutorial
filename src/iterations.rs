use leptos::*;

/// Displays a fixed length list
#[component]
pub fn StaticList(
    cx: Scope,
    /// The lenght of the list
    length: usize,
) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    let counter_buttons = counters
        .map(|(counter, set_counter)| {
            view! {
                cx,
                <li>
                    <button on:click= move |_| set_counter.update(|n| *n += 1)>
                        {counter}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();
    view! {
        cx,
        <ul>{counter_buttons}</ul>
    }
}

#[component]
pub fn DynamicList(
    cx: Scope,
    /// Initial length of the list
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(cx, id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(cx, initial_counters);

    let add_counter = move |_| {
        // create the signal for the new counter
        let sig = create_signal(cx, next_counter_id + 1);
        // add this counter to the list of the counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        cx,
        <div>
            <button on:click=add_counter>
                "Add counter"
            </button>
            <ul>
                <For
                    each= counters
                    key= |counter| counter.0
                    view= move |cx, (id, (count, set_count))|{
                        view! {
                            cx,
                            <li>
                                <button on:click= move |_| set_count.update(|n| *n += 1)>
                                {count}
                                </button>
                                <button on:click = move |_| {
                                    set_counters.update(|counters| {
                                        counters.retain(|(counter_id, _)| counter_id != &id )
                                    })
                                }>
                                "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
