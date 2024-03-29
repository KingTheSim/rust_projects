use leptos::*;

#[component]
fn Option2() -> impl IntoView {
    // here we create a signal in the root that can be consumed
    // anywhere in the app.
    let (count, set_count) = create_signal(0);
    // we'll pass the setter to specific components,
    // but provide the count itself to the whole app via context
    provide_context(count);

    view! {
        <h2>"Option 2: Passing Signals"</h2>
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <div>
            <FancyMath/>
            <ListItems/>
        </div>
    }
}

/// A button that increments our global counter.
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| set_count.update(|count| *count += 1)>
                "Increment Global Count"
            </button>
        </div>
    }
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
    // here we consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>().expect("there is a 'count' signal provided");
    let is_even = move || count() & 1 == 0;

    view! {
        <div>
            "The number "
            <strong>{count}</strong>
            { move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}

/// A component that shows a list of items generated from the global count.
#[component]
fn ListItems() -> impl IntoView {
    // again, consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>().expect("there is a 'count' signal provided");

    let squares = move || {
        (0..count())
            .map(|n| view! { <li>{n}<sup>"2"</sup> " is " {n * n}</li> })
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            <ul>{squares}</ul>
        </div>
    }
}

// Option #3: Create a Global State Struct
//
// You can use this approach to build a single global data structure
// that holds the state for your whole app, and then access it by
// taking fine-grained slices using `create_slice` or `create_memo`,
// so that changing one part of the state doesn't cause parts of your
// app that depend on other parts of the state to change.
#[derive(Default, Clone, Debug)]
struct GlobalState {
    count: u32,
    name: String,
}

#[component]
fn Option3() -> impl IntoView {
    // we'll provide a single signal that holds the whole state
    // each component will be responsible for creating its own "lens" into it
    let state = create_rw_signal(GlobalState::default());
    provide_context(state);

    view! {
        <h2>"Option 3: Passing Signals"</h2>
        <div>
            <h3>"Current Global State"</h3>
            <pre>
                {move || {
                    format!("{:#?}", state.get())
                }}
            </pre>
        </div>
        <div>
                <GlobalStateCounter/>
                <GlobalStateInput/>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! {
        <div>
            <button
                on:click=move |_| {
                    set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}

/// A component that updates the name in the global state.
#[component]
fn GlobalStateInput() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // this slice is completely independent of the `count` slice
    // that we created in the other component
    // neither of them will cause the other to rerun
    let (name, set_name) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.name.clone(),
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.name = n,
    );

    view! {
        <div>
            <input
                type="text"
                prop:value=name
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
            />
            <br/>
            <span>"Name is: " {name}</span>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {}

fn main() {
    leptos::mount_to_body(|| view! { <Option2/> <Option3/> })
}
